/*
Time:        46     85     75     82
Distance:   208   1412   1257   1410
 */
use std::os::raw::c_ulonglong;

fn race(time:c_ulonglong,distance:c_ulonglong) -> c_ulonglong{
    let mut ways:c_ulonglong = 0;

    for attempt in 0..=time{
        let score:c_ulonglong = (time-attempt) * (attempt);
        if distance < score{
            //println!("attemp:{}, score:{}",attempt,score);
            ways+=1;
        }
    }
    return ways;
}

fn main() {
    println!("{}", race(7,9));
    println!("{}",race(15,40));
    println!("{}",race(30,200));



    println!("{}",race(46,208)*race(85,1412)*race(75,1257)*race(82,1410));
    println!("{}",race(46857582,208141212571410));
}