use rand::Rng;
use std::time::Instant;
use std::{thread, time};
use std::sync::mpsc::{channel, Sender};
use std::time::Duration;
use std::io::Read;


fn main() {
    let (sender, receiver) = channel::<u8>();let (sender, receiver) = channel();
     let start = Instant::now();
     let lenght: i32 = 40;
     let mut score: usize = 1;
     let dino = 'è';
     let dinoy:u32 = 0; // 0=onground, 1 = first jump frame, 2 = second jump frame
     let cactus = 'à';
     let mut cactuspos: Vec<i32> = vec![];
     let mut cactuses: Vec<bool> = vec![];
     let mut screen = String::new();
     let mut now = start.elapsed().as_secs();
     let mut delay: u64 = 0;
     let mut rng = rand::thread_rng();
     let refreshdelay = time::Duration::from_millis(200);
     println!("Hello, world!");

     let handle = thread::spawn({
        let sender = sender.clone();
        move || {
            for byte in std::io::stdin().bytes() {
                let byte = byte.unwrap();
                if byte == 3 {
                    break;
                } else {
                    sender.send(()).unwrap();sender.send(()).unwrap();
                }
            }
        }
    });
     //start the game loop
     loop {

        match receiver.try_recv() {
            Ok(_) => {
                println!("YOU CLICKED");
            },
            Err(_) => {
                println!("Waiting for ohio invasion");
            }
        }
        
         gametick(
             lenght - <usize as TryInto<i32>>::try_into(score.to_string().len()).unwrap(),
             &mut cactuspos,
             &mut cactuses,
             score
         );
         now = start.elapsed().as_secs();
         if delay - now == 0 {
             delay = delay + rng.gen_range(2..5);
             spawncactus(&mut cactuspos, lenght);
         }
         thread::sleep(refreshdelay);
         refreshscreen(&cactuses, score, &mut screen, cactus, dino);
         score=score+1;
//         println!("{}",getch::getch());
     }
    handle.join().unwrap();
 }

 fn spawncactus(cactuspos: &mut Vec<i32>, lenghtdisplay: i32) {
     //    println!("summoning cacti....");
     cactuspos.insert(cactuspos.len(), lenghtdisplay);
     //    println!("{:?}", cactuspos);
 }

 fn gametick(lenghtdisplay: i32, cactuspos: &mut Vec<i32>, cactuses: &mut Vec<bool>, score:usize) {
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
     //    cactuses.push('{score}');
     //    cactuses.push('{score}');
     //println!("{:?}", cactuses);
 }

fn refreshscreen(cactuses:  &Vec<bool>, score: usize, screen:&mut String, cactuschar:char, dinochar:char) {
    *screen = String::from("");
    let mut index = 0;
    while index < cactuses.len() {
        if index == 1 {
            screen.push(dinochar);
        } else if cactuses[index] {
            screen.push(cactuschar);
        } else {
            screen.push('_');
        }
        index=index+1;
    }
    screen.push_str(&score.to_string());
    println!("{} {}","\x1B[2J\x1B[1;1H", screen);
//    println!("{}", screen.chars().count());
}