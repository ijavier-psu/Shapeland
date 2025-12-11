use rand::Rng;
use std::fmt;
use std::io;

const TIMESCALE: u32 = 5; //5 minute intervals
const CLOSING_TIME: u32 = 9 * 60;
//const num_dots: u32 = 500; //# of park guests total for the day

//s_rate is number of people to leave the queue per 5 minutes
struct Coaster {
    name: String,
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
        } else {
            self.queue = 0;
        }
    }

    //add a dot to the line + return its wait number
    fn add(&mut self) -> u32 {
        let t_loop = (self.queue / self.s_rate) + 1;
        self.queue += 1;
        t_loop
    }

    fn get_q_time(&self) -> u32 {
        self.queue / self.s_rate //simple wait time (no complex fast pass)
    }
}

#[derive(PartialEq, Clone)]
enum State {
    InQueue,
    Idle,
    InActivity,
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
    t_act: u32,
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

fn display_start_stats(fpass: bool, avg_dots: u32, e_dots: u32, verbose: bool, st: bool) {
    //NOTE: Unsafe code used because these values are only mutable at the beginning of the program
    //they are not to be changed after the main program begins
    println!("[0] FastPass enabled: {}", fpass);
    println!("[1] Number of average guests in the park: {} ", avg_dots);
    println!(
        "[2] Number of ride enthusiast guests (longer wait time tolerance) {}",
        e_dots
    );
    println!("[3] Verbose mode {}", verbose);
    println!("[4] Stat tracking mode {}", st);
    println!("[5] Done");
}

fn printshape() {
    println!("                      X");
    println!("                     X X");
    println!("                    X   X");
    println!("                   X     X ");
    println!("                  X       X");
    println!("                 X         X");
    println!("                X           X ");
    println!("               X             X ");
    println!("              X               X");
    //for i in 0..dots/10 {
    //    print!("*");
    //}
}

fn change_start_stats(
    fastpass: &mut bool,
    avg_dots: &mut u32,
    e_dots: &mut u32,
    verbose: &mut bool,
    st: &mut bool,
) {
    loop {
        display_start_stats(*fastpass, *avg_dots, *e_dots, *verbose, *st);
        let mut user_input = String::new();
        println!("Select a number to change: ");
        let _ = io::stdin().read_line(&mut user_input);
        let user_trim = user_input.trim();
        //println!("{}",user_trim);

        if user_trim == "0" {
            if *fastpass {
                println!("fastpass is now disabled");
                *fastpass = false;
            } else {
                println!("fastpass is now enabled");
                *fastpass = true;
            }
        } else if user_trim == "1" {
            let mut user_input = String::new();
            println!("Enter a number of guests (1-10000): ");
            let _ = io::stdin().read_line(&mut user_input);

            match user_input.trim().parse::<u32>() {
                Ok(number) => {
                    println!("You entered a valid number: {}", number);
                    *avg_dots = number;
                    // You can now use 'number' as an i32
                }
                Err(_) => {
                    println!("That's not a valid number!");
                    // Handle the error, e.g., prompt the user to try again
                }
            }
        } else if user_trim == "2" {
            let mut user_input = String::new();
            println!("Enter a number of ride crazy guests (1-10000): ");
            let _ = io::stdin().read_line(&mut user_input);

            match user_input.trim().parse::<u32>() {
                Ok(number) => {
                    println!("You entered a valid number: {}", number);
                    *e_dots = number;
                    // You can now use 'number' as an i32
                }
                Err(_) => {
                    println!("That's not a valid number!");
                    // Handle the error, e.g., prompt the user to try again
                }
            }
        } else if user_trim == "3" {
            if *verbose {
                println!("Verbose mode is now disabled");
                *verbose = false;
            } else {
                println!("Verbose mode is now enabled");
                *verbose = true;
            }
        } else if user_trim == "4" {
            if *st {
                println!("Stats mode is now disabled");
                *st = false;
            } else {
                println!("Stats mode is now enabled");
                *st = true;
            }
        } else if user_trim == "5" {
            break;
        } else {
            println!("Not recognized input. Try again");
        }
    }
}

fn main() {
    let mut clock = 0; //start at 9am
    let mut fastpass = true;
    let mut verbose = false;
    let mut stat_track = false;

    let mut avg_dots = 400;
    let mut e_dots = 100;

    let mut rides: [Coaster; 3] = [
        Coaster {
            name: String::from("triangle"),
            s_rate: 170,
            popularity: 10,
            queue: 0,
        },
        Coaster {
            name: String::from("square"),
            s_rate: 250,
            popularity: 8,
            queue: 0,
        },
        Coaster {
            name: String::from("circle"),
            s_rate: 125,
            popularity: 6,
            queue: 0,
        },
    ];

    printshape();
    println!("Welcome to Shapeland!");
    let mut user_input = String::new();

    println!("Here are the default values for your day at the park:");
    display_start_stats(fastpass, avg_dots, e_dots, verbose, stat_track);
    println!("Would you like to change anything before starting your adventure? (y/n)");
    let _ = io::stdin().read_line(&mut user_input);

    let user_trim = user_input.trim().to_lowercase();
    println!("{}", user_trim);
    if user_trim == "y" {
        change_start_stats(
            &mut fastpass,
            &mut avg_dots,
            &mut e_dots,
            &mut verbose,
            &mut stat_track,
        )
    }

    let mut rng = rand::thread_rng(); //rand # generator
    let num_dots = avg_dots + e_dots;

    //Average dot:
    let avg = (7, 24); //balk point = 120 min
    let e = (9, 36);

    //TODO: init dots
    let mut dots: Vec<Dot> = Vec::new();
    //for i in 0..num_dots{
    let mut i = 0;
    while avg_dots > 0 || e_dots > 0 {
        if avg_dots > 0 {
            dots.push(Dot {
                id: i,
                state: State::Idle,
                preference: avg.0,
                balk_point: avg.1,
                t_remaining: 0,
                t_ret: 0,
                t_act: 0,
                c_ret: 0,
                q_total: 0,
                c_ridden: 0,
                c_list: [0, 0, 0],
            });
            avg_dots -= 1;
            i += 1;
        }

        if e_dots > 0 {
            dots.push(Dot {
                id: i,
                state: State::Idle,
                preference: e.0,
                balk_point: e.1,
                t_remaining: 0,
                t_ret: 0,
                t_act: 0,
                c_ret: 0,
                q_total: 0,
                c_ridden: 0,
                c_list: [0, 0, 0],
            });
            e_dots -= 1;
            i += 1;
        }
    }

    println!("Hello, world!");
    let mut cur_queue = 0;
    let mut cur_act = 0;

    while clock < CLOSING_TIME {
        println!("Clock: {}, {} hours", clock, clock_hour(clock));

        for d in &mut dots {
            if verbose {
                println!("State of {} at start of time: {}", d.id, d.state);
            }

            match d.state {
                State::Idle => {
                    while d.state == State::Idle {
                        if d.t_ret == 1 {
                            //skip getting new activity if fastpass almost redeemable
                            break;
                        }

                        let dec = rng.gen_range(1..11); //1-10
                        if verbose {
                            println!("Dot {}'s range: {}", d.id, dec);
                        }

                        if dec <= d.preference {
                            if verbose {
                                println!("{} Wants to ride a ride!", d.id);
                            }
                            let mut ride_choice = rng.gen_range(0..24); //FIXME
                            if verbose {
                                println!("Ride choice range: {}", ride_choice);
                            }

                            for r in 0..3 {
                                if ride_choice > rides[r].popularity {
                                    ride_choice -= rides[r].popularity;
                                    continue;
                                }

                                //if wait time more than 30 min, check for fastpass
                                let queue_time = rides[r].get_q_time();
                                if verbose {
                                    println!("Wants to ride {}, with wait time {}", r, queue_time);
                                }
                                if queue_time > 6 {
                                    //FIXME
                                    //If i had more time, i'd keep track of coasters already checked

                                    //check if dot already has fastpass
                                    if fastpass && d.t_ret > 0 {
                                        if verbose {
                                            println!(
                                                "{} Already has fastpass for {}",
                                                d.id, d.c_ret
                                            );
                                        }
                                        //already has fast pass, check if standby line is short enough
                                        if queue_time < d.t_ret + 1 {
                                            d.state = State::InQueue;
                                            d.t_remaining = rides[r].add();
                                            d.c_ret = r;
                                            cur_queue += 1;
                                            if verbose {
                                                println!(
                                                    "d {}'s joined standby, wait time: {}",
                                                    d.id, d.t_remaining
                                                );
                                            }
                                        }
                                        break;
                                    }

                                    if fastpass && clock + queue_time < 540 {
                                        if verbose {
                                            println!("Getting Fast Pass");
                                        }
                                        d.t_ret = rides[r].add();
                                        d.c_ret = r;
                                        if verbose {
                                            println!("d {}'s return time: {}", d.id, d.t_ret);
                                        }
                                        break;
                                    }

                                    //check if standby queue is longer than balking point:
                                    if queue_time > d.balk_point {
                                        if verbose {
                                            println!(
                                                "line too long and no fastpass, returning to decision matrix"
                                            );
                                        }
                                        break;
                                    }
                                }

                                //Else, join standby line
                                d.state = State::InQueue;
                                d.t_remaining = rides[r].add();
                                d.c_ret = r;
                                cur_queue += 1;
                                if verbose {
                                    println!("d's wait time: {}", d.t_remaining);
                                }
                                break;
                            }
                        } else {
                            if verbose {
                                println!("Doing activity!");
                            }
                            let mut f_dec = 4;
                            if fastpass {
                                if verbose {
                                    println!("Fastpass time remaining: {}", d.t_ret);
                                }
                                f_dec = d.t_ret;

                                let dec = if f_dec > 1 {
                                    rng.gen_range(1..f_dec)
                                } else {
                                    1
                                };
                                d.state = State::InActivity;
                                d.t_remaining = dec;
                                cur_act += 1;
                            } else {
                                let dec = rng.gen_range(1..f_dec);
                                d.state = State::InActivity;
                                d.t_remaining = dec;
                                cur_act += 1;
                            }
                        }
                    } //while Idle
                }
                State::InQueue => {
                    //decrease time, if time == 1, state back to Idle
                    //++ total in queue time for dot
                    d.t_remaining -= 1;
                    d.q_total += 1;
                    if d.t_remaining == 0 {
                        d.state = State::Idle;
                        d.c_ridden += 1;
                        d.c_list[d.c_ret] += 1;
                        cur_queue -= 1;
                    }
                }
                State::InActivity => {
                    //decrease time
                    d.t_remaining -= 1;
                    if d.t_remaining == 0 {
                        d.t_act += 1;
                        d.state = State::Idle;
                        cur_act -= 1;
                    }
                }
            }

            if fastpass {
                if d.t_ret >= 1 {
                    d.t_ret -= 1;
                    if d.t_ret == 0 {
                        d.c_ridden += 1;
                        d.c_list[d.c_ret] += 1;

                        if verbose {
                            println!("{}'s Fast Pass Redeemed! Rode: {} : The {}", d.id, d.c_list[d.c_ret],rides[d.c_ret].name);
                        }
                        if verbose {
                            println!("pre-state: {}, t: {}", d.state, d.t_remaining);
                        }
                        if d.state == State::InQueue {
                            cur_queue -= 1;
                        }
                        if d.state == State::InActivity {
                            cur_act -= 1;
                        }
                        d.state = State::Idle;
                        continue;
                    }
                }
            }

            if verbose {
                println!("State of {} at end of time: {}", d.id, d.state);
            }
        }

        rides[0].service();
        println!("Number of dots in a queue: {}", cur_queue);
        println!("Number of dots in an activity: {}", cur_act);

        /*triangle.add();
        triangle.add();
        println!("Triangle queue: {}, time: {}",triangle.queue, get_q_time(&triangle));
        triangle.service();
        */
        clock += TIMESCALE;
    } //while park open

    let mut avg_queue = 0;
    let mut avg_ride = 0;

    for d in &dots {
        println!(
            "{} Total time (minutes) spent in queues: {}, activities done: {} times ridden {}: {}, {}: {}, {}: {}  ",
            d.id,
            d.q_total * TIMESCALE,
            d.t_act,
            rides[0].name,
            d.c_list[0],
            rides[1].name,
            d.c_list[1],
            rides[2].name,
            d.c_list[2]
        );
        if stat_track {
            avg_queue += d.q_total;
            avg_ride += d.c_ridden;
        }
    }

    if stat_track {
        avg_queue = avg_queue * TIMESCALE / num_dots;
        avg_ride = avg_ride / num_dots;
        println!("----\nAdditional stats:");
        println!("* Average total minutes spent in a queue: {}", avg_queue);
        println!("* Average number of rides rode: {}", avg_ride);
    }
}

fn clock_hour(c: u32) -> u32 {
    c / 60
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clock_test() {
        assert_eq!(1, clock_hour(60));
        assert_eq!(2, clock_hour(130));
    }
}
