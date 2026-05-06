use dioxus::prelude::*;

use crate::container::Container;

#[component]
pub fn AboutMe() -> Element {
    rsx! {
        Container {
            // TODO PUT LINKS TO STUFF
            p { margin_top: "0",
                "I'm a computer science enthusiast who loves tinkering around with systems
programming, compilers, and game development. I first discovered my passion for programming
during the Covid-19 pandemic when I built a series of games with Unity and PyGame to keep
myself busy."
            }

            p {
                "During my time at Stuyvesant High School, I continued programming and led the school's
First Tech Challenge robotics team to the world championships. I also volunteer mentored
middle school students through The Metis Project for 2 years, teaching them programming
fundamentals one-on-one. At the end of my junior year, I also landed an internship at
Bloomberg, where I have been working for the past few summers on a variety of projects.
    I've learnt about distributed systems, telemetry, and how to handle production scale
code."
            }

            p {
                "At Stony Brook, I've again joined the Robotics team, where I have been able to learn
more about ROS, embedded programming, and hardware communication protocols. I have also
    been working on a high performance system for indexing DNA databases with Prof. Michael
Ferdman. I've continued my passion for teaching others as well, mentoring a group of
students through building their own chess engine from scratch through the Stony Brook
Computing Society's Project Quack."
            }

            p { margin_bottom: "0",
                "Currently, I'm interested in a bunch of technologies, including Rust, Haskell, Typst,
and Vim. On the non-cs side of things, I love playing chess and basketball, solving
Rubik's cubes, and listening to music."
            }

        }
    }
}
