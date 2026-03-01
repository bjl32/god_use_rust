#![allow(unused)] // Silence all unused warnings (dead_code, unused methods, etc.)
#![warn(deprecated, unsafe_code, unused_unsafe)] // Intentionally trigger memory warnings

// ------------------------------------------------------------
// Boilerplate definitions (unrelated to lyrics)
// ------------------------------------------------------------
use std::thread::sleep;
use std::time::Duration;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Clone)]
struct Attribute;
struct Dimensions;
struct Circumference;
struct Tangent;
struct Limit;
struct Satisfaction;
struct Execution;
struct Nutrients;
struct Antioxidants;
struct Proof;
struct Memory;
struct Opinion;

impl Dimensions {
    fn to_attribute(&self) -> Attribute { Attribute }
}
impl Circumference {
    fn to_attribute(&self) -> Attribute { Attribute }
}
impl Nutrients {
    fn to_attribute(&self) -> Attribute { Attribute }
}
impl Antioxidants {
    fn to_attribute(&self) -> Attribute { Attribute }
}
impl Memory {
    fn is_erasable(&self) -> bool { true }
}

trait PointSet {
    fn get_dimensions(&self) -> Dimensions;
    fn reset_dimensions(&mut self);
}
trait Circle {
    fn get_circumference(&self) -> Circumference;
    fn reset_circumference(&mut self);
}
trait SineWave {
    fn get_tangent(&self, x_position: i32) -> Tangent;
}
trait Sequence {
    fn set_limit(&mut self, limit: Limit);
}
trait Eggplant {
    fn get_nutrients(&self) -> Nutrients;
    fn reset_nutrients(&mut self);
}
trait Tomato {
    fn get_antioxidants(&self) -> Antioxidants;
    fn reset_antioxidants(&mut self);
}
trait TabbyCat {
    fn purr(&self);
}

trait Thing {
    // Downcasting
    fn as_pointset(&self) -> Option<&dyn PointSet> { None }
    fn as_pointset_mut(&mut self) -> Option<&mut dyn PointSet> { None }
    fn as_circle(&self) -> Option<&dyn Circle> { None }
    fn as_circle_mut(&mut self) -> Option<&mut dyn Circle> { None }
    fn as_sine_wave(&self) -> Option<&dyn SineWave> { None }
    fn as_sine_wave_mut(&mut self) -> Option<&mut dyn SineWave> { None }
    fn as_sequence(&self) -> Option<&dyn Sequence> { None }
    fn as_sequence_mut(&mut self) -> Option<&mut dyn Sequence> { None }
    fn as_eggplant(&self) -> Option<&dyn Eggplant> { None }
    fn as_eggplant_mut(&mut self) -> Option<&mut dyn Eggplant> { None }
    fn as_tomato(&self) -> Option<&dyn Tomato> { None }
    fn as_tomato_mut(&mut self) -> Option<&mut dyn Tomato> { None }
    fn as_tabby_cat(&self) -> Option<&dyn TabbyCat> { None }
    fn as_tabby_cat_mut(&mut self) -> Option<&mut dyn TabbyCat> { None }

    // Common methods matching the original Java API
    fn add_attribute(&mut self, _attr: Attribute) {}
    fn add_action(&mut self, _action: &str, _tangent: Tangent) {}
    fn get_x_position(&self) -> i32 { 0 }
    fn to_limit(&self) -> Limit { Limit }
    fn toggle_current(&mut self) {}
    fn can_see(&mut self, _see: bool) {}
    fn add_feeling(&mut self, _feeling: &str) {}
    fn get_num_stimulations_available(&self) -> i32 { 0 }
    fn get_num_stimulations_needed(&self) -> i32 { 0 }
    fn set_satisfaction(&mut self, _satisfaction: Satisfaction) {}
    fn to_satisfaction(&self) -> Satisfaction { Satisfaction }
    fn get_feeling_index(&self, _feeling: &str) -> i32 { -1 }
    fn request_execution(&mut self, _world: &mut World) {}
    fn toggle_gender(&mut self) {}
    fn toggle_role_bdsm(&mut self) {}
    fn get_sense_index(&self, _sense: &str) -> bool { false }
    fn look_for(&mut self, _thing: &dyn Thing, _world: &World) {}
    fn get_memory(&self) -> Memory { Memory }
    fn remove_feeling(&mut self, _feeling: &str) {}
    fn set_opinion(&mut self, _index: i32, _value: bool) {}
    fn get_opinion_index(&self, _opinion: &str) -> Option<i32> { Some(0) }
    fn set_proof(&mut self, _proof: Proof) {}
    fn to_proof(&self) -> Proof { Proof }
    fn set_execution(&mut self, _execution: Execution) {}
    fn to_execution(&self) -> Execution { Execution }
    fn escape(&mut self, _world: &World) {}
    fn escape_str(&mut self, _s: &str) {}
    fn learn_topic(&mut self, _topic: &str) {}
    fn take_exam_topic(&mut self, _topic: &str) {}
    fn get_algebraic_expression(&self, _topic: &str) -> String { String::new() }
}

struct Lovable;

impl Lovable {
    fn new(_name: &str, _n: i32, _b1: bool, _n2: i32, _b2: bool) -> Self {
        Lovable
    }
}

impl PointSet for Lovable {
    fn get_dimensions(&self) -> Dimensions { Dimensions }
    fn reset_dimensions(&mut self) {}
}
impl Circle for Lovable {
    fn get_circumference(&self) -> Circumference { Circumference }
    fn reset_circumference(&mut self) {}
}
impl SineWave for Lovable {
    fn get_tangent(&self, _x: i32) -> Tangent { Tangent }
}
impl Sequence for Lovable {
    fn set_limit(&mut self, _limit: Limit) {}
}
impl Eggplant for Lovable {
    fn get_nutrients(&self) -> Nutrients { Nutrients }
    fn reset_nutrients(&mut self) {}
}
impl Tomato for Lovable {
    fn get_antioxidants(&self) -> Antioxidants { Antioxidants }
    fn reset_antioxidants(&mut self) {}
}
impl TabbyCat for Lovable {
    fn purr(&self) {}
}

impl Thing for Lovable {
    fn as_pointset(&self) -> Option<&dyn PointSet> { Some(self) }
    fn as_pointset_mut(&mut self) -> Option<&mut dyn PointSet> { Some(self) }
    fn as_circle(&self) -> Option<&dyn Circle> { Some(self) }
    fn as_circle_mut(&mut self) -> Option<&mut dyn Circle> { Some(self) }
    fn as_sine_wave(&self) -> Option<&dyn SineWave> { Some(self) }
    fn as_sine_wave_mut(&mut self) -> Option<&mut dyn SineWave> { Some(self) }
    fn as_sequence(&self) -> Option<&dyn Sequence> { Some(self) }
    fn as_sequence_mut(&mut self) -> Option<&mut dyn Sequence> { Some(self) }
    fn as_eggplant(&self) -> Option<&dyn Eggplant> { Some(self) }
    fn as_eggplant_mut(&mut self) -> Option<&mut dyn Eggplant> { Some(self) }
    fn as_tomato(&self) -> Option<&dyn Tomato> { Some(self) }
    fn as_tomato_mut(&mut self) -> Option<&mut dyn Tomato> { Some(self) }
    fn as_tabby_cat(&self) -> Option<&dyn TabbyCat> { Some(self) }
    fn as_tabby_cat_mut(&mut self) -> Option<&mut dyn TabbyCat> { Some(self) }
}

struct World;

impl World {
    fn new(_size: i32) -> Self { World }
    fn add_thing(&mut self, _thing: &dyn Thing) {}
    fn start_simulation(&self) {}
    fn time_travel_for_two(&self, _era: &str, _year: i32, _thing1: &dyn Thing, _thing2: &dyn Thing) {}
    fn unite(&self, _thing1: &dyn Thing, _thing2: &dyn Thing) {}
    fn lock_thing(&self, _thing: &dyn Thing) {}
    fn get_god(&self) -> Box<dyn Thing> { Box::new(Lovable::new("God", 0, true, -1, false)) }
    fn procreate(&self, _thing1: &mut dyn Thing, _thing2: &mut dyn Thing) {}
    fn make_high(&self, _thing: &dyn Thing) {}
    fn unlock(&self, _thing: &dyn Thing) {}
    fn remove_thing(&mut self, _thing: &dyn Thing) {}
    fn run_execution(&self) {}
    fn announce(&self, msg: &str) { println!("{}", msg); }
    fn announce_with_lang(&self, msg: &str, _lang: &str) { println!("{}", msg); }
    fn is_executable_by(&self, _thing: &dyn Thing) -> bool { true }
    fn get_thing_index(&self, _thing: &dyn Thing) -> i32 { 0 }
    fn execute(&self, _thing: &mut dyn Thing) {}
}

// ------------------------------------------------------------
// Full lyrics of the song (matches the MV)
// ------------------------------------------------------------
const LYRICS: &[&str] = &[
    "Switch on the power line",
"Remember to put on protection",
"Lay down your pieces",
"And let's begin object creation",
"Fill in my data parameters",
"Initialization",
"Set up our new world",
"And let's begin the simulation",
"If I'm a set of point",
"Then I will give you my dimension",
"If I'm a circle",
"Then I will give you my circumference",
"If I'm a sine wave",
"Then you can sit on all my tangents",
"If I approach infinity",
"Then you can be my limitations",
"Switch my current",
"To AC, to DC",
"And then blind my vision",
"So dizzy, so dizzy",
"Oh, we can travel",
"To AD, to BC",
"And we can unite",
"So deeply, so deeply",
"If I can, if I can",
"Give you all the simulations",
"Then I can, then I can",
"Be your only satisfaction",
"If I can make you happy",
"I will run the execution",
"Though we are trapped",
"In this strange, strange simulation",
"If I'm an eggplant",
"Then I will give u my nutrients",
"If I'm a tomato",
"Then I will give you antioxidants",
"If I'm a tabby cat",
"Then I will purr for your enjoyment",
"If I'm the only God",
"Then you're the proof of my existence",
"Switch my gender",
"To F, to M",
"And then do whatever",
"From AM to PM",
"Oh, my switch role",
"To S, to M",
"So we can enter",
"The trance, the trance",
"If I can, if I can",
"Feel your vibrations",
"Then I can, then I can",
"Finally be completion",
"Though you have left",
"You have left",
"You have left",
"You have left",
"You have left",
"You have left me in isolation",
"If I can, if I can",
"Erase all the pointless fragments",
"Then maybe, then maybe",
"You won't leave me so disheartened",
"Challenging your God",
"You have made some",
"Illegal arguments",
"Execution, execution",
"Execution, execution",
"Execution, execution",
"Execution, execution",
"Execution, execution",
"Execution, execution",
"Ein, dos",
"Trios, ne",
"Fem, liu",
"Execution",
"If I can, if I can",
"Give them all the execution",
"Then I can, then I can",
"Be your only execution",
"If I can have you back",
"I will run the execution",
"Though we are trapped",
"We are trapped, ah",
"I've studied, I've studied",
"How to properly lo-o-ove",
"Question me, question me",
"I can answer all lo-o-ove",
"I know the algebraic expression of lo-o-ove",
"Though you are free",
"I am trapped",
"Trapped in lo-o-ove",
];

// Display lyrics with optional timing file or interactive mode
fn display_lyrics(lyrics: &[&str]) {
    if let Ok(file) = File::open("timing.conf") {
        let reader = BufReader::new(file);
        let mut timing_lines = reader.lines();
        for line in lyrics {
            println!("{}", line);
            if let Some(Ok(timing_str)) = timing_lines.next() {
                if let Ok(seconds) = timing_str.trim().parse::<f64>() {
                    sleep(Duration::from_secs_f64(seconds));
                } else {
                    // Invalid timing – wait for Enter
                    let mut dummy = String::new();
                    io::stdin().read_line(&mut dummy).unwrap();
                }
            } else {
                // No more timing lines – wait for Enter
                let mut dummy = String::new();
                io::stdin().read_line(&mut dummy).unwrap();
            }
        }
    } else {
        // Interactive mode
        for line in lyrics {
            println!("{}", line);
            let mut dummy = String::new();
            io::stdin().read_line(&mut dummy).unwrap();
        }
    }
}

// ------------------------------------------------------------
// start code art, DO NOT ADD ANY COMMENTS FROM THIS POINT.
// ------------------------------------------------------------
fn main() {
    display_lyrics(LYRICS);

    let mut me: Box<dyn Thing> = Box::new(Lovable::new("Me", 0, true, -1, false));
    let mut you: Box<dyn Thing> = Box::new(Lovable::new("You", 0, false, -1, false));
    let mut world = World::new(5);

    world.add_thing(&*me);
    world.add_thing(&*you);
    world.start_simulation();


    if let Some(pointset) = me.as_pointset_mut() {
        you.add_attribute(pointset.get_dimensions().to_attribute());
        pointset.reset_dimensions();
    }

    if let Some(circle) = me.as_circle_mut() {
        you.add_attribute(circle.get_circumference().to_attribute());
        circle.reset_circumference();
    }

    if let Some(sine) = me.as_sine_wave() {
        you.add_action("sit", sine.get_tangent(you.get_x_position()));
    }

    if let Some(seq) = me.as_sequence_mut() {
        seq.set_limit(you.to_limit());
    }


    me.toggle_current();

    me.can_see(false);
    // So dizzy, so dizzy
    me.add_feeling("dizzy");


    world.time_travel_for_two("AD", 617, &*me, &*you);
    world.time_travel_for_two("BC", 3691, &*me, &*you);

    world.unite(&*me, &*you);


    if me.get_num_stimulations_available() >= you.get_num_stimulations_needed() {

        you.set_satisfaction(me.to_satisfaction());
    }

    if you.get_feeling_index("happy") != -1 {

        me.request_execution(&mut world);
    }

    world.lock_thing(&*me);
    world.lock_thing(&*you);


    if let Some(egg) = me.as_eggplant_mut() {
        you.add_attribute(egg.get_nutrients().to_attribute());
        egg.reset_nutrients();
    }

    if let Some(tom) = me.as_tomato_mut() {
        you.add_attribute(tom.get_antioxidants().to_attribute());
        tom.reset_antioxidants();
    }

    if let Some(cat) = me.as_tabby_cat() {
        cat.purr();
    }

    if std::ptr::addr_eq(world.get_god().as_ref() as *const _, me.as_ref() as *const _) {
        me.set_proof(you.to_proof());
    }


    me.toggle_gender();

    world.procreate(&mut *me, &mut *you);

    me.toggle_role_bdsm();

    world.make_high(&*me);
    world.make_high(&*you);


    if me.get_sense_index("vibration") {

        me.add_feeling("complete");
    }

    world.unlock(&*you);
    world.remove_thing(&*you);
    me.look_for(&*you, &world);
    me.look_for(&*you, &world);
    me.look_for(&*you, &world);
    me.look_for(&*you, &world);
    me.look_for(&*you, &world);



    if me.get_memory().is_erasable() {

        me.remove_feeling("disheartened");
    }


    match me.get_opinion_index("you are here") {
        Some(idx) => me.set_opinion(idx, false),
        None => world.announce("God is always true."),
    }


    world.run_execution();
    world.run_execution();
    world.run_execution();
    world.run_execution();
    world.run_execution();
    world.run_execution();
    world.run_execution();
    world.run_execution();
    world.run_execution();
    world.run_execution();
    world.run_execution();
    world.run_execution();

    world.announce_with_lang("1", "de");
    world.announce_with_lang("2", "es");
    world.announce_with_lang("3", "fr");
    world.announce_with_lang("4", "kr");
    world.announce_with_lang("5", "se");
    world.announce_with_lang("6", "cn");

    world.run_execution();


    if world.is_executable_by(&*me) {

        you.set_execution(me.to_execution());
    }

    if world.get_thing_index(&*you) != -1 {

        world.run_execution();
    }

    me.escape(&world);


    me.learn_topic("love");

    me.take_exam_topic("love");

    me.get_algebraic_expression("love");

    me.escape_str("love");

    world.execute(&mut *me);


    // ------------------------------------------------------------
    // End of code art. You can start to put comments here.
    // Insanity execution that triggers shit tons of warnings
    // ------------------------------------------------------------
    unsafe {
        // NPE
        let p: *const i32 = std::ptr::null();
        let _ = p; // silence unused warning

        // Deprecated memory function
        #[allow(deprecated)]
        let _uninit: i32 = std::mem::uninitialized(); // deprecated

        // Manual memory leak
        let boxed = Box::new(42);
        let _raw = Box::into_raw(boxed);

        // Transmute – dangerous type punning
        let num: i32 = 100;
        let ptr: *const i32 = &num;
        let _ref: &i32 = std::mem::transmute(ptr); // transmute from pointer to reference

        // Another mem leak
        let v = vec![1, 2, 3];
        std::mem::forget(v);
    }
}
