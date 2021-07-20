use rayon::iter::plumbing::{bridge_unindexed, UnindexedProducer};
use rayon::prelude::*;
use std::cell::UnsafeCell;
use std::convert::TryInto;
use std::fs::File;
use std::intrinsics::unlikely;
use std::io::{prelude::*, BufReader};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

const READER_CAPACITY: usize = 8 * 1024 * 1024;
const BUFFER_SIZE: usize = 64;
const MAXIMUM_NUMBER_OF_PRODUCERS: usize = 12;

fn lines_reader(reader: BufReader<File>, producers: Arc<RwLock<Vec<Arc<SPSCRingBuffer>>>>, comment_symbol: Option<String>) {
    let mut cells_to_populate_number: usize = 0;
    let mut write_index: usize = 0;
    let mut producer: Arc<SPSCRingBuffer> = {
        let producers = producers.read().unwrap();
        producers[0].clone()
    };

    for (line_number, line) in reader
        .lines()
        .map(|line| match line {
            Ok(l) => Ok(l),
            Err(e) => Err(e.to_string()),
        }).filter(move |line| match (line, comment_symbol) {
            (Ok(line), Some(cs)) => !line.is_empty() && !line.starts_with(&cs),
            (Ok(line), _) => !line.is_empty(),
            _ => true
        })
        .enumerate()
    {
        // If the number of cells to populate is zero,
        // we need to lock on the threads and find which
        // thread queue should be filled.
        'inner: while cells_to_populate_number == 0 {
            let producers = producers.read().unwrap();
            let producers_number = producers.len();
            for producer_index in (line_number..(line_number + producers_number))
                .map(|index| index % producers_number)
            {
                let (free_cells_number, this_write_index) =
                    producers[producer_index].get_free_cells_number_and_write_index();

                if free_cells_number != 0 {
                    producer = producers[producer_index].clone();
                    cells_to_populate_number = free_cells_number;
                    write_index = this_write_index;
                    break 'inner;
                }
            }
        }
        producer.write_unchecked(write_index, (line_number, line));
        write_index = (write_index + 1) % BUFFER_SIZE;
        // Surely by the end of this loop, we have
        // populated a cell.
        cells_to_populate_number -= 1;
        // If we have finished populating the cells,
        // we need to update the write index
        if cells_to_populate_number == 0 {
            producer.set_write_index_to_first_empty_cell(write_index)
        }
    }

    producer.set_write_index_to_first_empty_cell(write_index);

    let producers = producers.write().unwrap();
    for producer in producers.iter() {
        producer.stop();
    }
}

type IterType = (usize, Result<String, String>);

pub struct ParallelLinesWithIndex {
    file: File,
    comment_symbol: Option<String>,
    number_of_lines: Option<usize>,
    number_of_rows_to_skip: Option<usize>,
}

impl ParallelLinesWithIndex{
    pub fn new(path: &str) -> Result<ParallelLinesWithIndex, String> {
        let file = match File::open(path.clone()) {
            Ok(file) => Ok(file),
            Err(_) => Err(format!("Cannot open file {}", path)),
        }?;

        Ok(ParallelLinesWithIndex {
            file,
            number_of_lines: None,
            comment_symbol: None,
            number_of_rows_to_skip: None,
        })
    }

    pub fn set_skip_rows(&mut self, number_of_rows_to_skip: usize) {
        self.number_of_rows_to_skip = Some(number_of_rows_to_skip);
    }

    pub fn set_comment_symbol(&mut self, comment_symbol: Option<String>) {
        self.comment_symbol = comment_symbol;
    }
}

impl ParallelIterator for ParallelLinesWithIndex{
    type Item = IterType;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: rayon::iter::plumbing::UnindexedConsumer<Self::Item>,
    {
        let reader = BufReader::with_capacity(READER_CAPACITY, self.file);

        if let Some(rts) = self.number_of_rows_to_skip {
            for _ in 0..rts {
                let mut _buffer = String::new();
                reader.read_line(&mut _buffer).unwrap();
            }
        }

        let buffers = Arc::new(RwLock::new(vec![]));
        let producer = ParalellLinesProducerWithIndex::new(buffers.clone()).unwrap();

        thread::spawn(|| lines_reader(reader, buffers, self.comment_symbol));
        bridge_unindexed(producer, consumer)
    }

    fn opt_len(&self) -> Option<usize> {
        self.number_of_lines
    }
}

#[derive(Debug)]
pub struct SPSCRingBuffer {
    cells: UnsafeCell<[IterType; BUFFER_SIZE]>,
    write_idx: AtomicUsize,
    read_idx: AtomicUsize,
    stop: AtomicBool,
}

unsafe impl Sync for SPSCRingBuffer {}

#[macro_export]
/// Create a vector of atomic using a default value.
/// the syntax is:
/// `vec_atomic[AtomicTYPE; DEFAULT_VALUE; SIZE]`
macro_rules! init_array {
    [$default_value:expr ; $size:expr] => {{
        (0..$size)
            .map(|_| $default_value)
            .collect::<Vec<_>>().try_into().unwrap()
    }}
}

impl SPSCRingBuffer {
    pub fn new() -> SPSCRingBuffer {
        SPSCRingBuffer {
            cells: UnsafeCell::new(init_array![(0, Result::Ok(String::new())); BUFFER_SIZE]),
            write_idx: AtomicUsize::new(0),
            read_idx: AtomicUsize::new(0),
            stop: AtomicBool::new(false),
        }
    }

    pub fn read(&self) -> Option<IterType> {
        let idx = self.read_idx.load(Ordering::SeqCst);
        while unlikely(idx == self.write_idx.load(Ordering::SeqCst)) {
            if unlikely(self.is_stopped()) {
                return None;
            }
            std::thread::sleep(Duration::from_micros(10));
            // yield_now();
        }

        // get the id
        // get the str and replace the spot with an empty one
        let cells = unsafe { &mut *self.cells.get() };
        // TODO: possible optimizztion using MaybeInit
        let value = std::mem::replace(&mut cells[idx], (0, Ok(String::new())));

        self.read_idx
            .store((idx + 1) % BUFFER_SIZE, Ordering::SeqCst);

        Some(value)
    }

    pub fn stop(&self) {
        self.stop.store(true, Ordering::SeqCst);
    }

    pub fn is_stopped(&self) -> bool {
        self.stop.load(Ordering::SeqCst)
    }

    pub fn len(&self) -> usize {
        (BUFFER_SIZE + self.write_idx.load(Ordering::SeqCst) - self.read_idx.load(Ordering::SeqCst))
            % BUFFER_SIZE
    }

    /// returns how many free cells are **currently** available, the index of the first one.
    pub fn get_free_cells_number_and_write_index(&self) -> (usize, usize) {
        let w_idx = self.write_idx.load(Ordering::SeqCst);
        let r_idx = self.read_idx.load(Ordering::SeqCst);
        let free_cells_number = (BUFFER_SIZE - w_idx + r_idx - 1) % BUFFER_SIZE;
        (free_cells_number, w_idx)
    }

    pub fn write_unchecked(&self, idx: usize, value: IterType) {
        let cells = unsafe { &mut *self.cells.get() };
        cells[idx] = value;
    }

    pub fn set_write_index_to_first_empty_cell(&self, idx: usize) {
        self.write_idx.store(idx % BUFFER_SIZE, Ordering::SeqCst);
    }
}

#[derive(Debug)]
struct ParalellLinesProducerWithIndex {
    buffers_ref: Arc<RwLock<Vec<Arc<SPSCRingBuffer>>>>,
    buffer: Arc<SPSCRingBuffer>,
}

impl ParalellLinesProducerWithIndex {
    /// initialize and register the producer
    pub fn new(
        buffers_ref: Arc<RwLock<Vec<Arc<SPSCRingBuffer>>>>,
    ) -> Option<ParalellLinesProducerWithIndex> {
        let buffer = Arc::new(SPSCRingBuffer::new());
        //debug!("Trying to create a producer");
        {
            let mut producers = buffers_ref.write().unwrap();
            if producers.len() >= MAXIMUM_NUMBER_OF_PRODUCERS {
                //debug!("NVM");
                return None;
            }
            //debug!("Created the {}-th producer", producers.len());
            producers.push(buffer.clone());
        }

        Some(ParalellLinesProducerWithIndex {
            buffers_ref,
            buffer,
        })
    }
}

impl Iterator for ParalellLinesProducerWithIndex {
    type Item = IterType;

    fn next(&mut self) -> Option<Self::Item> {
        self.buffer.read()
    }
}

impl UnindexedProducer for ParalellLinesProducerWithIndex {
    type Item = IterType;

    /// Split the file in two approximately balanced streams
    fn split(self) -> (Self, Option<Self>) {
        // Check if it's reasonable to split the stream
        if self.buffer.is_stopped() {
            return (self, None);
        }

        let new = ParalellLinesProducerWithIndex::new(self.buffers_ref.clone());

        (self, new)
    }

    fn fold_with<F>(self, folder: F) -> F
    where
        F: rayon::iter::plumbing::Folder<Self::Item>,
    {
        folder.consume_iter(self)
    }
}
