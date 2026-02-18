use getgpuname::get_gpu_name;

fn main() {
    if let Some(gpu) = get_gpu_name() {
        println!("{}", gpu)
    } else {
        panic!()
    }
}
