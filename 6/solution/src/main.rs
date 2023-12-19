/*
Time:        46     85     75     82
Distance:   208   1412   1257   1410
 */
use std::os::raw::c_ulonglong;

fn race(time:c_ulonglong, record:c_ulonglong) -> c_ulonglong{
    let mut ways:c_ulonglong = 0;

    for attempt in 0..=time{
        let score:c_ulonglong = (time-attempt) * (attempt);
        if record < score{
            //println!("attemp:{}, score:{}",attempt,score);
            ways+=1;
        }
    }
    return ways;
}

fn main() {
    println!("Time: 7,\tDistance: 208\t->\t{}", race(7,9));
    println!("Time:85,\tDistance:1412\t->\t{}",race(15,40));
    println!("Time:75,\tDistance:1257\t->\t{}",race(30,200));

    println!("Pt1->\t\t\t{}",race(46,208)*race(85,1412)*race(75,1257)*race(82,1410));
    println!("Pt2->\t\t\t{}",race(46857582,208141212571410));
}