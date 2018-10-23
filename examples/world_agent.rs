/*
WORLD - AGENT

In this example will we try to solve the following problem:

    A an AI agent has a mode and a goal.
    The mode consists of 3 states: Happy, Sad and Angry.
    The goal consists of 2 states: BeGood and BeBad.
    The world is a grid consisting of 4x4 squares.
    The agent can move up, right, down or left.
    Find a way to enumerate all changes of states,
    whether it is a different mood, a different goal,
    or a different position in the world.

A directed context space can be used to describe changes of states.
First, we figure out how to encode moods and goals.

If we use the dimensions `[mood, goal]`, which are 18 states,
then we don't know where the agent is or which direction it will go.

If we use the dimensions `[mood, goal, direction]`, which are 144 states,
then we get duplicates because we do not want to know change of direction.

So, there is a trick:
Instead of changing direction, we add a direction "go nowhere".
This means get 5 directions, but it can be factored out into a dimension.

What we got now is `([mood, goal], direction)`.
It gives us 90 states.

However, for every change in mode or goal, there is now all directions.
We would like to remove this redundancy, so there is either
a change in mode or goal, or direction.

Again, there is a trick:
By using `([mood, goal, 2], direction/2)` with the original 4 directions,
there is a change `0 -> 1` and `1 -> 0` in the directed context space.
This bit tells us whether to use `up/down` or `right/left`.
It means we need only to pick between the other two in the dimension.

However, this gives us 96 states, which is worse than before.
The reason is that there are two direction bits for every change in
mood or goal when we only need one.

So, we need to select between `[mood, goal]` or `direction`.
However, we need to keep the `mood` and `goal` state when going in a direction.
This means we have to select either `{[mood, goal], [mood, goal, direction]}`.
The first option is a directed context space,
while the second option is an N-dimensional space.

To do this one can use the `Either` space.
This reduces the number of states to 42.

Finally, we put the grid inside a N-dimensional space:
({[mood, goal], [mood, goal, direction]}, [4, 4])

This gives us a total of 672 states.

*/

extern crate discrete;

use discrete::*;

fn main() {
    let mood = 3;
    let goal = 2;
    let direction = 4;

    let context: (Either<DirectedContext, DimensionN>, DimensionN) = Construct::new();
    let dim = ((vec![mood, goal], vec![mood, goal, direction]), vec![4, 4]);
    let count = context.count(&dim);
    println!("count: {:?}", count);

    let mut pos = (Select::Fst((vec![0, 0], 1, 1)), vec![0, 0]);
    // println!(" {:?}", context.to_index(&dim, &pos));
    for x in 0..5 {
        context.to_pos(&dim, x, &mut pos);
        println!("\n{}/{} {}% {:?}", x, count, (100.0 * (x as f64) / (count as f64)).round(), pos);
        desc(&pos);
    }
}

fn desc(pos: &(Select<(Vec<usize>, usize, usize), Vec<usize>>, Vec<usize>)) {
    let (mood, goal, change, new_value, direction) = match pos.0 {
        Select::Fst(ref pos) => (pos.0[0], pos.0[1], pos.1, pos.2, None),
        Select::Snd(ref pos) => (pos[0], pos[1], 2, pos[2], Some(pos[2])),
    };

    let x = pos.1[0];
    let y = pos.1[1];
    print!("Mood: ");
    desc_mood(mood);
    print!("Goal: ");
    desc_goal(goal);
    if let Some(direction) = direction {
        desc_direction(direction);
    }
    if change == 0 {
        print!("Change mood to: ");
        desc_mood(new_value);
    } else if change == 1 {
        print!("Change goal to: ");
        desc_goal(new_value);
    }
    println!("Position: ({}, {})", x, y);
}

fn desc_mood(mood: usize) {
    match mood {
        0 => println!("Happy"),
        1 => println!("Sad"),
        2 => println!("Angry"),
        _ => {}
    }
}

fn desc_goal(goal: usize) {
    match goal {
        0 => println!("BeGood"),
        1 => println!("BeBad"),
        _ => {}
    }
}

fn desc_direction(direction: usize) {
    print!("Change position: ");
    match direction {
        0 => println!("go up"),
        1 => println!("go right"),
        2 => println!("go down"),
        3 => println!("go left"),
        _ => {}
    }
}
