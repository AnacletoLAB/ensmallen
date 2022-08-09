use gpu_models::*;

pub fn add_one(vals: &[f32]) -> Result<Vec<f32>, GPUError> {
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
    let mut ptx = gpu.load_ptx(PTX_SOURCE)?;
    //
    println!("{}", PTX_SOURCE.len());
    // get a function from the compiled code
    let kernel = ptx.get_kernel("compute_first_order_line")?;
    
    // // allocate a gpu buffer and copy data from the host
    // let buffer = gpu.buffer_from_slice::<f32>(vals)?;
    // // set the parallelizzation specs
    // let grid = Grid::default().set_block_x(1024)?;

    // // launch the function with the args
    // gpu.launch_kernel(&kernel, &grid, args![
    //     buffer.as_device_ptr(),
    //     buffer.len(),
    // ])?;
    // // wait for the gpu to finish
    // gpu.synchronize()?;

    // // copy the results back from the gpu to the host
    // let result = buffer.to_vec()?;
    Ok(vec![0.0])
}

fn main() {
    println!("{:?}", add_one(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0]).unwrap());
}  
