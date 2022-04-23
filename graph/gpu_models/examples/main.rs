use gpu_models::*;

pub fn add_one(vals: &[u32]) -> Result<Vec<u32>, GPUError> {
    println!("{:?}", get_driver_version());
    // get all the devices in the system
    let devices = Device::get_devices()?;
    // get info about this device
    println!("{:#4?}", devices);
    // we use the first device
    let device = devices[0];

    // setup this device for computation
    let mut gpu = GPU::new(device)?;
    // load our compiled code
    let mut ptx = gpu.load_ptx(PTX)?;
    // get a function from the compiled code
    let kernel = ptx.get_kernel("add_one")?;
    
    // allocate a gpu buffer and copy data from the host
    let buffer = gpu.buffer_from_slice::<u32>(vals)?;
    // set the parallelizzation specs
    let grid = Grid::default().set_block_x(1024)?;

    // launch the function with the args
    gpu.launch_kernel(&kernel, &grid, args![
        buffer.as_device_ptr(),
        buffer.len(),
    ])?;
    // wait for the gpu to finish
    gpu.synchronize()?;

    // copy the results back from the gpu to the host
    let result = buffer.to_vec()?;
    Ok(result)
}

fn main() {
    println!("{:?}", add_one(&[1, 2, 3, 4, 5, 6, 7]).unwrap());
}  
