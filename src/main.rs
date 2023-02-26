use rand::Rng;
use std::sync::mpsc::channel;
//use std::time::Instant;
use std::{thread, time};
//use std::time::Duration;
use std::io::Read;

extern crate nix;
use nix::sys::termios;
use term_size;
fn main() {
    //     let (sender, receiver) = channel::<u8>();
    let (sender, receiver) = channel();
    //    let start = Instant::now();
    let lenght = match term_size::dimensions() {
        Some((width, _)) => width-1,
        None => 40,
    };
    let mut score: usize = 1;
    let dino = 'è';
    let mut dinoy: i32 = 0; // 0=onground, 1 = first jump frame, 2 = second jump frame, 3 third jump frame
    let cactus = 'à';
    let mut cactuspos: Vec<i32> = vec![];
    let mut cactuses: Vec<bool> = vec![];
    let mut screen = String::new();
    //    let mut now: u64;
    let refreshdelay = 200;
    let mut rng = rand::thread_rng();
    let mut delay: u64 = rng.gen_range(1100..2500)/refreshdelay;
    let gameover = String::from("gameovermessage");
    //     println!("Hello, world!");

    let handle = thread::spawn({
        let sender = sender.clone();
        move || {
            // Querying original as a separate, since `Termios` does not implement copy
            let orig_term = termios::tcgetattr(0).unwrap();
            let mut term = termios::tcgetattr(0).unwrap();
            // Unset canonical mode, so we get characters immediately
            term.local_flags.remove(termios::LocalFlags::ICANON);
            // Don't generate signals on Ctrl-C and friends
            term.local_flags.remove(termios::LocalFlags::ISIG);
            // Disable local echo
            term.local_flags.remove(termios::LocalFlags::ECHO);
            termios::tcsetattr(0, termios::SetArg::TCSADRAIN, &term).unwrap();
            for byte in std::io::stdin().bytes() {
                let byte = byte.unwrap();
                if byte == 3 {
                    break;
                } else {
                    sender.send(()).unwrap();
                }
            }
            termios::tcsetattr(0, termios::SetArg::TCSADRAIN, &orig_term).unwrap();

        }
    });
    //start the game loop
    loop {
        match receiver.try_recv() {
            Ok(_) => {
                if dinoy == 0 {
                    dinoy = 1;
                }
            }
            Err(_) => {}
        }

        gametick(
            lenght - score.to_string().len(),
            &mut cactuspos,
            &mut cactuses,
            &mut dinoy,
        );
//        now = start.elapsed().as_secs();

        if delay == 0 {
            spawncactus(&mut cactuspos, lenght);
            delay = rng.gen_range(1100..2500)/refreshdelay;
        }
        thread::sleep(time::Duration::from_millis(refreshdelay));
        refreshscreen(&cactuses, score, &mut screen, cactus, dino, dinoy);
        if screen == gameover {
            println!(
                "{}game over (are  you really that bad) {}",
                "\x1B[2J\x1B[1;1H", score
            );
            //            println!("game over {}", score);
            break;
        }
        score = score + 1;
        delay = delay - 1;
        //         println!("{}",getch::getch());
    }
    handle.join().unwrap();
}

fn spawncactus(cactuspos: &mut Vec<i32>, lenghtdisplay: usize) {
    //    println!("summoning cacti....");
    cactuspos.insert(cactuspos.len(), lenghtdisplay.try_into().unwrap());
    //    println!("{:?}", cactuspos);
}

fn gametick(
    lenghtdisplay: usize,
    cactuspos: &mut Vec<i32>,
    cactuses: &mut Vec<bool>,
    dinoy: &mut i32,
) {
    //    println!("tick!");
    cactuses.clear();
    let mut index: usize = 1;
    let mut indexcacti = 0;
    while index <= lenghtdisplay.try_into().unwrap() {
        if cactuspos.len() > indexcacti {
            if index == cactuspos[indexcacti].try_into().unwrap() {
                cactuses.push(true);
                indexcacti = indexcacti + 1;
            } else {
                cactuses.push(false);
            }
        } else {
            cactuses.push(false);
        }
        index = index + 1;
    }
    index = 1;
    while cactuspos.len() >= index.try_into().unwrap() {
        if cactuspos.len() > 0 {
            if cactuspos[0] == 0 {
                cactuspos.remove(0);
            }
        }
        cactuspos[index - 1] = cactuspos[index - 1] - 1;
        index = index + 1;
    }
    //jump management
    if *dinoy == 1 {
        *dinoy = 2;
    } else if *dinoy == 2 {
        *dinoy = 3;
    } else if *dinoy == 3 {
        *dinoy = 4;
    } else if *dinoy == 4 {
        *dinoy = 0;
    }

//         println!("{}", dinoy);
    //    cactuses.push('{score}');
    //    cactuses.push('{score}');
    //println!("{:?}", cactuses);
}

fn refreshscreen(
    cactuses: &Vec<bool>,
    score: usize,
    screen: &mut String,
    cactuschar: char,
    dinochar: char,
    dinoy: i32,
) {
    *screen = String::from("");
    let mut index = 0;
    while index < cactuses.len() {
        if index == 1 && dinoy == 0 {
            screen.push(dinochar);
        } else if cactuses[index] {
            screen.push(cactuschar);
        } else {
            screen.push('_');
        }
        index = index + 1;
    }
    screen.push_str(&score.to_string());
    println!("{} {}", "\x1B[2J\x1B[1;1H", screen);
    //    println!("{}", screen);
    if cactuses[1] && dinoy == 0 {
        *screen = String::from("gameovermessage");
    }
    //    println!("{}", screen.chars().count());
}
