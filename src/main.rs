use rand::Rng;
use std::time::{Duration, Instant};
use std::{thread, time};

fn main() {
    let start = Instant::now();
    let lenght:i32 = 40;
    let mut score = 0;
    let dino = 'è';
    let cactus = 'à';
    let mut cactuspos:Vec<i32> = vec![];
    let mut now = start.elapsed().as_secs();
    let mut delay:u64 = 0;
    let mut rng = rand::thread_rng();
    let refreshdelay = time::Duration::from_millis(200);
    println!("Hello, world!");
    //start the game loop
    loop {
        gametick(lenght, &mut cactuspos);
        now = start.elapsed().as_secs();
        if delay-now == 0 {
            delay = delay+rng.gen_range(2..5);
            spawncactus(&mut cactuspos);
        }
        thread::sleep(refreshdelay);
    }
}

fn spawncactus(cactuspos:&mut Vec<i32>) {
    println!("summoning cacti....");
    cactuspos.insert(cactuspos.len(), 40);
    println!("{:?}", cactuspos);
}

fn gametick(lenghtdisplay:i32, cactuspos:&mut Vec<i32>) {
    println!("tick!");
    let mut screen:Vec<char> = vec![];
    let mut index = 1;
    let mut indexcacti = 0;
    while index<=lenghtdisplay {
//        println!("{}", index);
//        println!("{}", indexcacti);
        if cactuspos.len() > indexcacti {
            if index == cactuspos[indexcacti] {
                screen.push('1');
            } else {
                screen.push('-');
            }
        } else {
            screen.push('-');
        }
        indexcacti=indexcacti+1;
        index = index+1;
    }
    index = 1;
    while cactuspos.len() >= index.try_into().unwrap() {
        cactuspos[index] = cactuspos[index] - 1;
    }
    println!("{:?}", screen);
}