
fn main() {

    let org_arr = [1,2,3,5,6,8,10,11];

    let sub_arr = [6,8,10];

    let abc = sub_array(&org_arr, &sub_arr);

    println!("Sub_arr is a subarray of org_arr : {}", abc)
}

fn sub_array(arr1: &[i32], arr2: &[i32]) -> bool {
        if arr1.len() < arr2.len() {
            return false;
        }

        'laplai: for m in 0..arr2.len() {
            if arr1.contains(&arr2[m]) {
                continue 'laplai;
            } return false;
        } return true;
}