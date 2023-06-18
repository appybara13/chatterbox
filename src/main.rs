use chatter::chatter;

use crate::player::Player;

mod player;

fn main() {
    let basic = chatter! {r#"
# start
There is a door in front of you.
You attempt to open the door, but it doesn't budge.
- Open the door. -> open
- Kick down the door. -> kick
- Pick the lock. -> pick
- Call out "Is anyone in there?" -> call
- Walk way. -> walk

# open
You try to open the door, but it is locked.
-> start

# kick
You take a few steps back, and gather your strength.
You charge at the door, and kick it with all your might.
Nothing changes, except your foot now hurts.
-> start

# pick
You attempt to pick the lock, but fail.
-> start

# call
You call out "Is anyone in there?"
...
No response.
-> start
    
# walk
You turn around, and walk away.
    "#};

    let player = Player::new(basic);
    player.run();
}
