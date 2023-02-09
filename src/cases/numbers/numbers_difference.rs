    // Find the missing term in an Arithmetic Progression
// An Arithmetic Progression is defined as one in which there
//  is a constant difference between the consecutive terms of
//  a given series of numbers. You are provided with consecutive
//   elements of an Arithmetic Progression. There is however one 
//   hitch: exactly one term from the original series is missing
//    from the set of numbers which have been given to you. The 
//    rest of the given series is the same as the original AP. 
//    Find the missing term.

// You have to write a function that receives a list, 
// list size will always be at least 3 numbers. The missing 
// term will never be the first or last one.
fn find_missing(seq: &[i32]) -> i32 {
    let mut sq = seq.to_vec();
    let mut first_derivative = vec![];
    let mut boolean_second_derivative = vec![];
    let mut common_difference = seq[1]-seq[0];
    
    
    if seq.len() < 4 {
        let mut s = vec![];
        for i in 0..2 {
            s.push(seq[i+1]-seq[i])
        }
        let min = s.iter().min().unwrap();
        
        for i in 0..2 {
            let difference = seq[i+1] - seq[i];
            if !(difference - min==0) {
                return seq[i]+min
            }
        }
        
    } else {
        for i in 0..3 {
            first_derivative.push(seq[i+1]-seq[i]); 
        }

    }
    
    for i in 0..3 {
        boolean_second_derivative.push(first_derivative[i%2] == first_derivative[((i)%2)+1])
    }

    for i in 0..3 {
        if !boolean_second_derivative[i%2] {
            common_difference = first_derivative[(i+1)%2]
        }
    }
    

    while let Some((current, rest)) = sq.split_first_mut() {
        let difference = rest[0] - *current;      
        if !(difference - common_difference==0) {
            return *current + common_difference;
        }
        sq = rest.to_vec();
    }

    0
}


// best solution found:

fn find_missing(seq: &[i32]) -> i32 {
    (seq.first().unwrap() + seq.last().unwrap()) * (seq.len() + 1) as i32 / 2 - seq.iter().sum::<i32>()
}
