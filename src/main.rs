use rand::Rng;
use std::fmt;

const TIMESCALE: u32 = 5; //5 minute intervals
const NUM_DOTS: u32 = 5; //# of park guests total for the day

//s_rate is number of people to leave the queue per 5 minutes
struct Coaster {
    s_rate: u32,
    popularity: u32,
    queue: u32,
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
}

//dot type
enum D_type {
    Avg
}

enum State {
    In_Queue,
    Idle,
    In_Activity
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            State::In_Queue => write!(f, "In Queue"),
            State::In_Activity => write!(f, "In Activity"),
            State::Idle => write!(f, "Idle"),
        }
    }
}
//can dtype be an Enum? or how else to have similiar base types?
//Maybe need to make Dots tuples instead idk

//preference: 1-10
//t_remaining: multiples of TIMESCALE 
struct Dot {
   d_type: D_type,
   state: State,
   preference: u32,
   t_remaining: u32,
   q_total: u32,
}

//impl Dot {
//}

//FIXME: make function of Coaster?
fn get_q_time(c: &Coaster) -> u32 {
    ( c.queue / c.s_rate ) // * TIMESCALE
}




fn main() {
    let mut clock = 0; //start at 9am

    //TODO: init rides
    let mut triangle = Coaster {
            s_rate: 1,
            popularity: 10,
            queue: 0,
    };

    let mut rng = rand::thread_rng(); //rand # generator

    //TODO: init dots
    let mut dots: Vec<Dot> = Vec::new();
    dots.push(Dot { d_type: D_type::Avg, state: State::Idle, preference: 7, t_remaining: 0, q_total: 0 });

    println!("Hello, world!");
    
    while clock < 540 {
        println!("Clock: {}, {} hours",clock, clock_hour(clock));

        //service queues before putting dots
        for d in &mut dots {
            println!("State at start of time: {}",d.state);
            match d.state {
            State::Idle => {
                let dec = rng.gen_range(1..11); //1-10
                println!("Dot's range: {}",dec);


                if dec <= d.preference {
                    println!("Riding ride!");
                    //FIXME: Check ride's q_time first
                    d.state = State::In_Queue;
                    d.t_remaining = triangle.add();
                    println!("d's wait time: {}",d.t_remaining);
                }
                else {
                    println!("Doing activity!");
                    let dec = rng.gen_range(1..4);
                    d.state = State::In_Activity;
                    d.t_remaining = dec;
                }
            },
            State::In_Queue => {
                //decrease time, if time == 1, state back to Idle
                //++ total in queue time for dot
                d.t_remaining -= 1;
                d.q_total += 1;
                if d.t_remaining == 0 {
                    d.state = State::Idle
                }
            },
            State::In_Activity => {
                //decrease time
                d.t_remaining -= 1;
                if d.t_remaining == 0 {
                    d.state = State::Idle
                }
            }

            }
            
            println!("State at end of time: {}",d.state);
        }

        triangle.service();

        /*triangle.add();
        triangle.add();
        println!("Triangle queue: {}, time: {}",triangle.queue, get_q_time(&triangle));
        triangle.service();
        */
        clock += TIMESCALE;
    } //while park open

    println!("Total time in queue: {}",dots[0].q_total);

}


fn clock_hour(c: u32) -> u32 {
    c / 60
}
