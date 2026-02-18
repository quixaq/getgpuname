use getgpuname::get_gpu_name_from_id;

fn main() {
    if let Some(gpu) = get_gpu_name_from_id(4098, 29822, Some(7586), Some(54389)) {
        println!("{}", gpu);
    } else {
        panic!();
    }
}
