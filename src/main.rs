use rand::Rng;
use std::fmt;

const TIMESCALE: u32 = 5; //5 minute intervals
const NUM_DOTS: u32 = 500; //# of park guests total for the day


//s_rate is number of people to leave the queue per 5 minutes
struct Coaster {
    name: String,
    s_rate: u32,
    popularity: u32,
    queue: u32,
    fast_queue: u32,
}

//FIXME: Would be nice if whole thing didnt need to be mutable
//What if we had the queue be a separate variable?
impl Coaster {
    fn service(&mut self) {
        if self.queue >= self.s_rate {
            self.queue -= self.s_rate;
        }
        else {
            self.queue = 0;
        }
    }

    //add a dot to the line + return its wait number
    fn add(&mut self) -> u32 {
      let t_loop = ( self.queue / self.s_rate ) + 1;
      self.queue += 1;  
      t_loop
    }

    fn get_q_time(&self) -> u32{
        self.queue / self.s_rate //simple wait time (no complex fast pass)
    }
}


 #[derive(PartialEq)]
 #[derive(Clone)]
enum State {
    InQueue,
    Idle,
    InActivity
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            State::InQueue => write!(f, "In Queue"),
            State::InActivity => write!(f, "In Activity"),
            State::Idle => write!(f, "Idle"),
        }
    }
}
//can dtype be an Enum? or how else to have similiar base types?
//Maybe need to make Dots tuples instead idk

//preference: 1-10
//t_remaining: time remaining in current queue
//t_ret: time to return to fastpass ride
//c_ret: coaster to return to
#[derive(Clone)]
struct Dot {
   id: u32,
   state: State,
   preference: u32,
   balk_point: u32,
   t_remaining: u32,
   t_ret: u32,
   c_ret: usize,
   q_total: u32,
   c_ridden: u32,
   c_list: [u32; 3],
}

//impl Dot {
//}

//FIXME: make function of Coaster?
/* 
fn get_q_time(c: &Coaster) -> u32 {
    c.queue / c.s_rate // * TIMESCALE
}
*/

fn displayStartStats(fpass: &mut bool){
    //NOTE: Unsafe code used because these values are only mutable at the beginning of the program
    //they are not to be changed after the main program begins
    println!("[0] FastPass enabled: {}",fpass);
    println!("[1] Number of guests in the park: {} ",NUM_DOTS);

}



fn main() {
    let mut clock = 0; //start at 9am
    let mut fastpass = true;

    //TODO: init rides
    let mut rides: [Coaster; 3] = [
        Coaster {
            name: String::from("triangle"),
            s_rate: 10,
            popularity: 10,
            queue: 0,
            fast_queue: 0
    },
        Coaster {
            name: String::from("square"),
            s_rate: 5,
            popularity: 8,
            queue: 0,
            fast_queue: 0
    },
        Coaster {
            name: String::from("circle"),
            s_rate: 4,
            popularity: 6,
            queue: 0,
            fast_queue: 0
    }
    ];
    

    println!("Welcome to Shapeland!");
    println!("Here are the default values for your day at the park:");
    displayStartStats(&mut fastpass);
    println!("Would you like to change anything before starting your adventure? (y/n");
    
    

    let mut rng = rand::thread_rng(); //rand # generator


    //Average dot:
    let avg = (7,30);

    //TODO: init dots
    let mut dots: Vec<Dot> = Vec::new();
    for i in 0..NUM_DOTS{
        dots.push(Dot { id: i, state: State::Idle, preference: avg.0, balk_point: avg.1, t_remaining: 0, t_ret: 0, c_ret: 0, q_total: 0, c_ridden: 0, c_list: [0,0,0] });
    }

    println!("Hello, world!");
    
    while clock < 540 {
        println!("Clock: {}, {} hours",clock, clock_hour(clock));
        
        for d in &mut dots {
            println!("State of {} at start of time: {}",d.id, d.state);

            match d.state {

            State::Idle => {
                while d.state == State::Idle {

                let dec = rng.gen_range(1..11); //1-10
                println!("Dot's range: {}",dec);


                if dec <= d.preference {
                    println!("Riding ride!");
                    let mut ride_choice = rng.gen_range(0..24); //FIXME
                    println!("Ride choice: {}",ride_choice);
                    
                    for r in 0..3 {
                        if ride_choice > rides[r].popularity {
                            ride_choice -= rides[r].popularity;
                            continue;
                        }

                        //if wait time more than 30 min, check for fastpass
                        let queue_time = rides[r].get_q_time();
                        println!("wants to ride {}, wait time {}",r,queue_time);
                        if queue_time > 6 {
                            //FIXME
                            //If i had more time, i'd keep track of coasters already checked
                            
                            //check if dot already has fastpass
                            if fastpass && d.t_ret > 0 {
                                println!("{} Already has fastpass for {}",d.id,d.c_ret);
                                //already has fast pass, check if standby line is short enough
                                if queue_time < d.t_ret {
                                    d.state = State::InQueue;
                                    d.t_remaining = rides[r].add();
                                    d.c_ret = r;
                                    println!("d {}'s wait time: {}",d.id,d.t_remaining);
                                }
                                break;
                            }
                            
                            if fastpass && clock + queue_time < 540 {
                                println!("Getting Fast Pass");
                                d.t_ret = rides[r].add();
                                d.c_ret = r;
                                println!("d {}'s return time: {}",d.id, d.t_ret);
                                break;
                            }

                            //check if standby queue is longer than balking point:
                            if queue_time > d.balk_point {
                                println!("line too long and no fastpass, returning to decision matrix");
                                break;
                            }
                        }

                        //Else, join standby line
                        d.state = State::InQueue;
                        d.t_remaining = rides[r].add();
                        d.c_ret = r;
                        println!("d's wait time: {}",d.t_remaining);
                        break;
                    }

                }
                else {
                    println!("Doing activity!");
                    let mut f_dec = 4;
                    if fastpass{
                        println!("Fastpass time remaining: {}",d.t_ret);

                        let dec = if f_dec > 1 { 
                            rng.gen_range(1..f_dec)
                        }
                        else {
                            1
                        };
                        d.state = State::InActivity;
                        d.t_remaining = dec;
                        
                    }
                    else {
                        let dec = rng.gen_range(1..f_dec);
                        d.state = State::InActivity;
                        d.t_remaining = dec;
                    }
                }
                } //while Idle
            },
            State::InQueue => {
                //decrease time, if time == 1, state back to Idle
                //++ total in queue time for dot
                d.t_remaining -= 1;
                d.q_total += 1;
                if d.t_remaining == 0 {
                    d.state = State::Idle;
                    d.c_ridden += 1;
                    d.c_list[d.c_ret] += 1;
                }
            },
            State::InActivity => {
                //decrease time
                d.t_remaining -= 1;
                if d.t_remaining == 0 {
                    d.state = State::Idle;
                }
            }

            }

            if fastpass{
                if d.t_ret >= 1{
                    d.t_ret -= 1;
                    if d.t_ret == 0 {
                        d.c_ridden += 1;
                        d.c_list[d.c_ret] += 1;
                        println!("{} Fast Pass Redeemed! Rode: {}",d.id,d.c_list[d.c_ret]);
                        d.state = State::Idle;
                        continue;
                    }
                }
            }
            
            println!("State of {} at end of time: {}",d.id,d.state);
        }

        rides[0].service();

        

        /*triangle.add();
        triangle.add();
        println!("Triangle queue: {}, time: {}",triangle.queue, get_q_time(&triangle));
        triangle.service();
        */
        clock += TIMESCALE;
    } //while park open


    for d in &dots {
        println!("{} Total time in queue: {}, times ridden triangle: {} ",d.id,d.q_total,d.c_list[0]);
    }

}


fn clock_hour(c: u32) -> u32 {
    c / 60
}
