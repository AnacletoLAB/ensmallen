use memchr;
#[cfg(target_os = "linux")]
use nix::fcntl::*;
use rayon::iter::plumbing::{bridge_unindexed, UnindexedProducer};
use rayon::prelude::*;
use std::fs::File;
use std::io::{prelude::*, BufReader, ErrorKind, SeekFrom};
#[cfg(target_os = "linux")]
use std::os::unix::io::AsRawFd;

pub const READER_CAPACITY: usize = 1 << 17;

type IterType = (usize, Result<String, String>);

pub struct ParallelLinesWithIndex<'a> {
    path: &'a str,
    file: File,
    comment_symbol: Option<String>,
    number_of_lines: Option<usize>,
    number_of_rows_to_skip: Option<usize>,
    buffer_size: usize,
    max_producers: usize,
}

impl<'a> ParallelLinesWithIndex<'a> {
    pub fn new(path: &'a str) -> Result<ParallelLinesWithIndex<'a>, String> {
        let file = match File::open(path.clone()) {
            Ok(file) => Ok(file),
            Err(_) => Err(format!("Cannot open file {}", path)),
        }?;

        Ok(ParallelLinesWithIndex {
            path: path,
            file,
            number_of_lines: None,
            comment_symbol: None,
            number_of_rows_to_skip: None,
            buffer_size: READER_CAPACITY,
            max_producers: num_cpus::get(),
        })
    }

    pub fn set_buffer_size(&mut self, buffer_size: usize) {
        self.buffer_size = buffer_size;
    }

    pub fn set_max_producers(&mut self, max_producers: usize) {
        self.max_producers = max_producers;
    }

    pub fn set_skip_rows(&mut self, number_of_rows_to_skip: usize) {
        self.number_of_rows_to_skip = Some(number_of_rows_to_skip);
    }

    pub fn set_comment_symbol(&mut self, comment_symbol: Option<String>) {
        self.comment_symbol = comment_symbol;
    }
}

impl<'a> ParallelIterator for ParallelLinesWithIndex<'a> {
    type Item = IterType;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: rayon::iter::plumbing::UnindexedConsumer<Self::Item>,
    {
        #[cfg(target_os = "linux")]
        let _ = posix_fadvise(
            self.file.as_raw_fd(),
            0,
            0,
            PosixFadviseAdvice::POSIX_FADV_SEQUENTIAL,
        );

        // Create the file reader
        let mut reader = BufReader::with_capacity(self.buffer_size, self.file);
        // Skip the first rows (as specified by the user)
        if let Some(rts) = self.number_of_rows_to_skip {
            for _ in 0..rts {
                let mut _buffer = String::new();
                let result_bytes_read = reader.read_line(&mut _buffer);
                match result_bytes_read {
                    Ok(bytes_read) => {
                        // Reached End Of File
                        if bytes_read == 0 {
                            break;
                        }
                    }
                    Err(_errot) => {}
                }
            }
        }

        // Create the first producer
        let producer = ParalellLinesProducerWithIndex {
            path: self.path,
            file: reader,
            line_count: 0,
            modulus_mask: 0,
            depth: 0,
            remainder: 0,
            maximal_depth: (self.max_producers as f64).log2().ceil() as usize,
            buffer_size: self.buffer_size,
            comment_symbol: self.comment_symbol,
        };
        bridge_unindexed(producer, consumer)
    }

    fn opt_len(&self) -> Option<usize> {
        self.number_of_lines
    }
}

#[derive(Debug)]
struct ParalellLinesProducerWithIndex<'a> {
    path: &'a str,
    file: BufReader<File>,
    line_count: usize,
    modulus_mask: usize,
    remainder: usize,
    maximal_depth: usize,
    depth: usize,
    buffer_size: usize,
    comment_symbol: Option<String>,
}

impl<'a> ParalellLinesProducerWithIndex<'a> {
    #[inline]
    fn get_modulus(&self) -> usize {
        self.modulus_mask + 1
    }
}

impl<'a> Iterator for ParalellLinesProducerWithIndex<'a> {
    type Item = IterType;

    fn next(&mut self) -> Option<Self::Item> {
        // read a line
        let mut line_buffer = Vec::with_capacity(128);
        // In this counter we will sum the number of characters
        // that have been read for the buffered line.
        let mut line_number_of_characters_read = 0;
        'outer: loop {
            let (line_is_finished, number_of_characters_read) = {
                // We get the next batch of characters from the buffer.
                let mut available = match self.file.fill_buf() {
                    Ok(n) => n,
                    Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
                    Err(_) => {
                        return Some((
                            self.line_count,
                            Err("Something went wrong reading the file".to_string()),
                        ))
                    }
                };
                let mut number_of_characters_read = 0;

                // If this is a file with '\n\r' end of lines, it is not impossible
                // that the read bits start with the carriage return symbol.
                // We need to handle this use case as follows:
                if !available.is_empty() && available[0] == b'\r' {
                    available = &available[1..];
                    number_of_characters_read += 1;
                }
                if let Some(comment_symbol) = self.comment_symbol.as_deref() {
                    if available.starts_with(comment_symbol.as_bytes()) {
                        // We deincrease the number of characters that have been found.
                        let _ = self.line_count.saturating_sub(1);
                        number_of_characters_read += comment_symbol.len();
                        available = &available[comment_symbol.len()..];
                    }
                }
                let mut line_is_finished = false;
                while let Some(pos) = memchr::memchr(b'\n', available) {
                    // update the count of how many bytes from the buffer we have parsed
                    number_of_characters_read += pos + 1;
                    // We either have finished searching for the correct delimiter
                    // and we can finally start to grow our string from this delimiter
                    // or alternatively we need to parse the currently loaded characters
                    if (self.line_count & self.modulus_mask) != self.remainder {
                        available = &available[(pos + 1)..];
                        if !available.is_empty() && available[0] == b'\r' {
                            available = &available[1..];
                            number_of_characters_read += 1;
                        }
                        // We increase the number of characters that have been found.
                        self.line_count += 1;
                        if let Some(comment_symbol) = self.comment_symbol.as_deref() {
                            if available.starts_with(comment_symbol.as_bytes()) {
                                // We deincrease the number of characters that have been found.
                                self.line_count -= 1;
                                number_of_characters_read += comment_symbol.len();
                                available = &available[comment_symbol.len()..];
                            }
                        }
                        continue;
                    }
                    // If we are now finally in the correct line, we can grow
                    // our line buffer.
                    line_is_finished = true;
                    // We return a tuple containing a boolean
                    // that represents we are done with preparing
                    // this line of the file and the number of characters
                    // that have been read.
                    line_number_of_characters_read += pos + 1;
                    // Note that we EXCLUDE the separator from the line, so we don't
                    // need to check to remove it afterwards.
                    line_buffer.extend_from_slice(&available[..pos]);
                    // We increase the number of characters that have been found.
                    self.line_count += 1;
                    break;
                }
                if !line_is_finished {
                    // Otherwise we can continue to read onward.
                    // We get the number of characters that have been read.
                    number_of_characters_read += available.len();
                    // If the number of delimiters that we need to find still is
                    // just one, that is the final delimiter, we need to store
                    // these characters into the string buffer.
                    if (self.line_count & self.modulus_mask) == self.remainder {
                        line_buffer.extend_from_slice(available);
                        line_number_of_characters_read += available.len();
                    }
                }
                // We return a tuple containing a boolean
                // that represents we are not yet done with preparing
                // this line of the file and the number of characters
                // that have been read.
                (line_is_finished, number_of_characters_read)
            };
            // We consume a number of characters from the file
            // equal to the number of characters we have read
            self.file.consume(number_of_characters_read);
            // If either the line is finished or the number
            // of characters read is zero, we are done.
            if line_is_finished || number_of_characters_read == 0 {
                break 'outer;
            }
        }

        if line_number_of_characters_read == 0 {
            None
        } else {
            Some((self.line_count - 1, unsafe {
                Ok(String::from_utf8_unchecked(line_buffer))
            }))
        }
    }
}

impl<'a> UnindexedProducer for ParalellLinesProducerWithIndex<'a> {
    type Item = IterType;

    /// Split the file in two approximately balanced streams
    fn split(mut self) -> (Self, Option<Self>) {
        // Check if it's reasonable to split the stream
        if self.depth >= self.maximal_depth.saturating_sub(1) {
            return (self, None);
        }

        let file =
            File::open(self.path.clone()).expect(&format!("Could not open the file {}", self.path));

        #[cfg(target_os = "linux")]
        let _ = posix_fadvise(
            file.as_raw_fd(),
            0,
            0,
            PosixFadviseAdvice::POSIX_FADV_SEQUENTIAL,
        );

        // Create a copy of the file reader of the father
        let mut new_file = BufReader::with_capacity(self.buffer_size, file);

        // Updated its position to the same byte in the file as the father.
        new_file
            .seek(SeekFrom::Start(self.file.stream_position().expect(
                "Could not read the file pointer position in the file.",
            )))
            .expect("Could seek the new file to the position of the old one.");

        // Since we only do binary splits, the modulus will always be a power of
        // two. So when checking if the current line should be retrieved,
        // we don't need to use the slower % but we can use the faster &.
        // In particular, if modulus is a power of two, it holds that:
        // x % modulus == x & (modulus - 1)
        // So what we call modulus_mask is (modulus - 1), and thus to get it back
        // we can do modulus = (modulus_mask + 1).
        //
        // Here we want to double the modulus, so we compute:
        // new_modulus_mask = (2 * modulus) - 1 = (modulus_mask * 2) + 1
        // which can be rewritten in terms of shift and or for faster
        let new_modulus_mask = (self.modulus_mask << 1) | 1;

        // We need to split in half, we need to return half the lines in
        // each child. Therefore we must double the modulus and we need to
        // offset one of the two childs.
        //
        // It can be proven that the following two properties holds:
        // x mod n = (x mod 2*n) \cup (x + n mod 2*n)
        // (x mod 2*n) \cap (x + n mod 2*n) = null
        // |(x mod 2*n)| == |(x + n mod 2*n)
        // So these are two perfect half-splits of the range.
        //
        // # Example:
        // Suppose that we have mod: 4, rem: 1, we will sign with `_` the lines
        // to skip, and with `$` the lines to return:
        //
        // Line idx: 0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15
        // Father:   _  $  _  _  _  $  _  _  _  $  _  _  _  $  _  _
        // Split1:   _  $  _  _  _  _  _  _  _  $  _  _  _  _  _  _
        // Split2:   _  _  _  _  _  $  _  _  _  _  _  _  _  $  _  _
        //
        // So we get the two childs with:
        // mod 8 = 2 * old_mod, rem 1 = old_rem
        // mod 8 = 2 * old_mod, rem 5 = old_rem + old_modulus
        //
        let new = ParalellLinesProducerWithIndex {
            path: self.path.clone(),
            line_count: self.line_count,
            file: new_file,
            modulus_mask: new_modulus_mask,
            remainder: self.get_modulus() + self.remainder,
            buffer_size: self.buffer_size,
            comment_symbol: self.comment_symbol.clone(),
            depth: self.depth + 1,
            maximal_depth: self.maximal_depth,
        };

        // Update the father modulus so that the lines are equally splitted
        self.modulus_mask = new_modulus_mask;
        self.depth += 1;

        // Returns the two new producers
        (self, Some(new))
    }

    fn fold_with<F>(self, folder: F) -> F
    where
        F: rayon::iter::plumbing::Folder<Self::Item>,
    {
        folder.consume_iter(self)
    }
}
