use num::integer::sqrt;

// There's a prime number hiding in our array of integers!
// The function below tries to find the prime number by checking each element,
// and finding its divisor. If none is found, then it's a prime number and
// its search ends!

// But it seems that its search never does end, when there's clearly a
// prime number there. Fix the function so that it returns the prime number.

fn main() {
    let numbers = [36, 25, 49, 3, 64, 16, 9];
    let prime = get_prime(numbers);
    println!("The first prime number in the array is {}.", prime);
}

fn get_prime(arr: [i32; 7]) -> i32 {

    let mut i = 0;
    'outer: loop {

        let mut n = 2;
        loop {
            
            if arr[i] % n == 0 {
                if arr[i] == 2 {
                    break 'outer;
                }
                i += 1;
                break;
            }

            if n >= sqrt(arr[i]) {
                break 'outer;
            }
            
            n += 1;
        }
    }
    println!("The first prime number in the array is {}.", arr[i]);
    arr[i]
}
