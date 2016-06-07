#[macro_use] extern crate conrod;
extern crate piston_window;
extern crate serial;
extern crate libc;

#[allow(unused_imports)]
use conrod::{
    color,
    Button,
    Canvas,
    Circle,
    Color,
    Colorable,
    DropDownList,
    EnvelopeEditor,
    Frameable,
    Labelable,
    NumberDialer,
    Point,
    Positionable,
    Slider,
    Sizeable,
    Text,
    TextBox,
    Theme,
    Toggle,
    Widget,
    WidgetMatrix,
    XYPad,
    TitleBar,
};
use piston_window::{EventLoop, Glyphs, OpenGL, PistonWindow, UpdateEvent, WindowSettings};
use std::sync::mpsc;

use serial::SerialPort;

use std::time::Duration;

type Backend = (<piston_window::G2d<'static> as conrod::Graphics>::Texture, Glyphs);
type Ui = conrod::Ui<Backend>;
type UiCell<'a> = conrod::UiCell<'a, Backend>;

//#[link(name="rs_jni_pipe", kind="static")]
mod rs_2_java {
    use libc;
    #[allow(dead_code)]
    #[link(name="jvm")]
    extern "system" {
        pub fn rs_java_vm_create() -> libc::c_int;
        
        pub fn rs_java_vm_destroy() -> libc::c_int;
        
        pub fn solution(
            facelets  : *const u8,
            max_depth : libc::c_int,
            probe_max : libc::c_longlong,
            probe_min : libc::c_longlong,
            verbose   : libc::c_int,
            manuever  : *mut u8,
            man_len   : *mut libc::size_t
        );
        
        pub fn next(
            probe_max : libc::c_longlong,
            probe_min : libc::c_longlong,
            verbose   : libc::c_int,
            manuever  : *mut u8,
            man_len   : *mut libc::size_t
        );
        
        pub fn is_inited() -> libc::c_int;
        
        pub fn number_of_probes() -> libc::c_longlong;
        
        pub fn length() -> libc::c_int;
        
        pub fn init();
    }
}

#[allow(dead_code)]
mod min2phase {
    use libc;
    use ::rs_2_java;
    pub fn solution(facelets : &[u8; 54], max_depth : i32, probe_max : i64, probe_min : i64, verbose : i32) -> String {
        unsafe {
            let mut dst = Vec::with_capacity(512);
            let psrc = facelets.as_ptr();
            let pdst = dst.as_mut_ptr();
            let mut n : libc::size_t = 1;
            
            rs_2_java::solution(
                facelets.as_ptr(),
                max_depth as libc::c_int,
                probe_max as libc::c_longlong,
                probe_min as libc::c_longlong,
                verbose as libc::c_int,
                pdst,
                &mut n
            );
            
            println!("N: {}", n);
            
            dst.set_len(n);
            
            String::from_utf8(dst).unwrap()
        }
    }
    
    pub fn next(probe_max : i64, probe_min : i64, verbose : i32) -> String {
        unsafe {
            let mut dst = Vec::with_capacity(512);
            let pdst = dst.as_mut_ptr();
            let mut n : libc::size_t = 0;
            
            rs_2_java::next(
                probe_max as libc::c_longlong,
                probe_min as libc::c_longlong,
                verbose as libc::c_int,
                pdst,
                &mut n
            );
            
            dst.set_len(n);
            
            String::from_utf8(dst).unwrap()
        }
    }
    
    pub fn is_inited() -> bool {
        unsafe{
            match rs_2_java::is_inited() {
                1 => { true },
                0 => { false },
                -1 =>{ panic!(); /*error of some sort*/},
                _ => {panic!();} /*something very bad happened here*/
            }
        }
    }
    
    pub fn number_of_probes() -> i64 {
        unsafe {
            rs_2_java::number_of_probes() as i64
        }
    }
    
    pub fn length() -> i32 {
        unsafe {
            rs_2_java::length() as i32
        }
    }
    
    pub fn init() {
        unsafe {
            rs_2_java::init()
        }
    }
}


fn encode_seq(ms : &[u8]) -> Vec<u8> {
    let mut r :Vec<u8> = vec!['s' as u8,ms.len() as u8];
    r.extend(ms.iter().cloned());
    r
}

struct Control {
    port        : Option<Box<SerialPort>>,
    port_name   : String,
    seq         : String,
    facelets    : [u8; 54],
}

impl Control {
    fn new() -> Control {
        let mut t = [0u8;54];
        for i in 1..6 {
            for j in (i*9)..((i+1)*9) {
                t[j] = i as u8;
            }
        }
        Control {
            port        : None,
            port_name   : "".to_string(),
            seq         : "".to_string(),
            facelets    : t,
        }
    }
    
    fn send_via_port<'a>(&mut self, bytes : &'a [u8]) {
        match &mut self.port {
            &mut None => (),
            &mut Some(ref mut s) => {
                match s.reconfigure(&|settings| {
                    match settings.set_baud_rate(serial::Baud9600) { Err(e) => {return Err(e);}, _ => () }
                    settings.set_char_size(serial::Bits8);
                    settings.set_parity(serial::Parity::ParityNone);
                    settings.set_stop_bits(serial::StopBits::Stop1);
                    settings.set_flow_control(serial::FlowControl::FlowNone);
                    Ok(())
                }) {
                    Err(e) => {/*handle error*/},
                    _ => ()
                }
            }
        }
            
        match &mut self.port {
            &mut None => (),
            &mut Some(ref mut p) => {
                match p.write(bytes) {
                    Err(e) => {/*Handle Error*/},
                    Ok(_) => ()
                }
            }
        }
    }
    
    fn set_widgets<'a>(&'a mut self, ui: &'a mut UiCell) {
        
        { //Main Canvas and Title Bar
            Canvas::new()
                .frame(0.0)
                .color(color::rgb_bytes(0x78, 0x90, 0x9C))
                .scroll_kids()
                .set(CANVAS, ui);
                
            Canvas::new()
                .frame(0.0)
                .w_of(CANVAS)
                .h(64.0)
                .mid_top_of(CANVAS)
                .color(color::rgb_bytes(0xB7, 0x1C, 0x1C))
                .set(TITLEBAR, ui);
            
            Text::new("3x3x3 Cube Puzzle Solver")
                .middle_of(TITLEBAR)
                .font_size(32)
                .color(color::rgb_bytes(0xff,0xff,0xff))
                .set(TITLE, ui);
        }
        
        { //Port Dialog
            Canvas::new()
                .frame(0.0)
                .top_right_with_margin_on(CANVAS, 16.0)
                .down_from(TITLEBAR, 16.0)
                .w(256.0)
                .h(112.0)
                .color(color::rgb_bytes(0x02, 0x77, 0xBD))
                .set(PORT_DIALOG, ui);
                
            Canvas::new()
                .frame(0.0)
                .w_of(PORT_DIALOG)
                .h(48.0)
                .mid_top_of(PORT_DIALOG)
                .color(color::rgb_bytes(0x00, 0xB0, 0xFF))
                .set(PORT_TITLE_BAR, ui);
            
            Text::new("Port")
                .middle_of(PORT_TITLE_BAR)
                .font_size(32)
                .color(color::rgb_bytes(0x00, 0x00, 0x00))
                .set(PORT_TITLE, ui);
            
            TextBox::new(&mut self.port_name)
                .frame(0.0)
                .bottom_left_with_margin_on(PORT_DIALOG, 16.0)
                .w(96.0)
                .h(32.0)
                .react(|_string: &mut String|{})
                .color(color::rgb_bytes(0xE1, 0xF5, 0xFE))
                .set(PORT_INPUT, ui);
                
            Button::new()
                .frame(0.0)
                .bottom_right_with_margin_on(PORT_DIALOG, 16.0)
                .label("SET")
                .w(96.0)
                .h(32.0)
                .react(||{
                    println!("{}", self.port_name);
                    self.port = match serial::open(&self.port_name) {
                        Ok(p)  => Some(Box::new(p)),
                        Err(_) => None
                    };
                })
                .color(color::rgb_bytes(0x00, 0x91, 0xEA))
                .set(SET_PORT, ui);
        }
        
        { //Manual Control
            Canvas::new()
                .frame(0.0)
                .bottom_right_with_margin_on(CANVAS, 16.0)
                .w(256.0)
                .h(352.0)
                .color(color::rgb_bytes(0x43, 0xA0, 0x47))
                .set(MANUAL_DIALOG, ui);
                
            Canvas::new()
                .frame(0.0)
                .w_of(MANUAL_DIALOG)
                .h(48.0)
                .mid_top_of(MANUAL_DIALOG)
                .color(color::rgb_bytes(0x00, 0xE6, 0x76))
                .set(MANUAL_TITLE_BAR, ui);
            
            Text::new("Manual Control")
                .middle_of(MANUAL_TITLE_BAR)
                .font_size(32)
                .color(color::rgb_bytes(0x10,0x10,0x10))
                .set(MANUAL_TITLE, ui);
            
            let sidi = ['U', 'R', 'F', 'D', 'L', 'B'];
            let seqi = [0b0000, 0b0001, 0b0010, 0b0011, 0b0100, 0b0101];
            for i in 0..6 {
                
                Button::new()
                    .frame(0.0)
                    .label(std::str::from_utf8(&[sidi[i] as u8]).unwrap())
                    .w(32.0)
                    .h(32.0)
                    .top_left_with_margin_on(MANUAL_DIALOG, 16.0)
                    .down_from(if i == 0 {MANUAL_TITLE_BAR} else {MANUAL_BIG + (i - 1)}, 16.0)
                    .color(color::rgb_bytes(0x00, 0xC8, 0x53))
                    .react(|| {
                        println!("SEND: {}+", sidi[i]);
                        self.send_via_port(&[sidi[i] as u8, '+' as u8]);
                    })
                    .set(MANUAL_BIG + i, ui);
                    
                
                Button::new()
                    .frame(0.0)
                    .label(std::str::from_utf8(&[sidi[i] as u8 ^ (1<<5)]).unwrap())
                    .w(32.0)
                    .h(32.0)
                    .right_from(MANUAL_BIG + i, 16.0)
                    //.down_from(if i == 0 {MANUAL_TITLE_BAR} else {MANUAL_SMALL + (i - 1)}, 16.0)
                    .color(color::rgb_bytes(0x00, 0xC8, 0x53))
                    .react(|| {
                        println!("SEND: {}+", sidi[i]);
                        self.send_via_port(&[sidi[i] as u8 ^ (1<<5), '+' as u8]);
                    })
                    .set(MANUAL_SMALL + i, ui);
                    
                    
                Button::new()
                    .frame(0.0)
                    .label(std::str::from_utf8(&[sidi[i] as u8, '\'' as u8]).unwrap())
                    .w(32.0)
                    .h(32.0)
                    .top_right_with_margin_on(MANUAL_DIALOG, 16.0)
                    .down_from(if i == 0 {MANUAL_TITLE_BAR} else {MANUAL_BIG_PRIME + (i - 1)}, 16.0)
                    .color(color::rgb_bytes(0x00, 0xC8, 0x53))
                    .react(|| {
                        println!("SEND: {}+", sidi[i]);
                        self.send_via_port(&[sidi[i] as u8, '-' as u8]);
                    })
                    .set(MANUAL_BIG_PRIME + i, ui);
                    
                
                Button::new()
                    .frame(0.0)
                    .label(std::str::from_utf8(&[sidi[i] as u8 ^ (1<<5), '\'' as u8]).unwrap())
                    .w(32.0)
                    .h(32.0)
                    .left_from(MANUAL_BIG_PRIME + i, 16.0)
                    //.down_from(if i == 0 {MANUAL_TITLE_BAR} else {MANUAL_SMALL + (i - 1)}, 16.0)
                    .color(color::rgb_bytes(0x00, 0xC8, 0x53))
                    .react(|| {
                        println!("SEND: {}+", sidi[i]);
                        self.send_via_port(&[sidi[i] as u8 ^ (1<<5), '-' as u8]);
                    })
                    .set(MANUAL_SMALL_PRIME + i, ui);
            }
            
        }
        
        { //Enter a space, comma, or semicolon delimited sequence of moves
            Canvas::new()
                .frame(0.0)
                .mid_bottom_with_margin_on(CANVAS, 16.0)
                .w(704.0)
                .h(112.0)
                .color(color::rgb_bytes(0x02, 0x77, 0xBD))
                .set(SEQ_DIALOG, ui);
                
            Canvas::new()
                .frame(0.0)
                .w_of(SEQ_DIALOG)
                .h(48.0)
                .mid_top_of(SEQ_DIALOG)
                .color(color::rgb_bytes(0x00, 0xB0, 0xFF))
                .set(SEQ_TITLE_BAR, ui);
            
            Text::new("Sequence")
                .middle_of(SEQ_TITLE_BAR)
                .font_size(32)
                .color(color::rgb_bytes(0x00, 0x00, 0x00))
                .set(SEQ_TITLE, ui);
            
            TextBox::new(&mut self.seq)
                .frame(0.0)
                .bottom_left_with_margin_on(SEQ_DIALOG, 16.0)
                .w(704.0-96.0-48.0)
                .h(32.0)
                .react(|_string: &mut String|{})
                .color(color::rgb_bytes(0xE1, 0xF5, 0xFE))
                .set(SEQ_TEXT_BOX, ui);
                
            Button::new()
                .frame(0.0)
                .bottom_right_with_margin_on(SEQ_DIALOG, 16.0)
                .label("SEND")
                .w(96.0)
                .h(32.0)
                .react(||{
                    let mut tmp = vec!['s' as u8];
                    tmp.push(self.seq.split(" ").filter(|&x| {
                        match x {
                            "U" =>  true,
                            "U'" => true,
                            "R" =>  true,
                            "R'" => true,
                            "F" =>  true,
                            "F'" => true,
                            "D" =>  true,
                            "D'" => true,
                            "L" =>  true,
                            "L'" => true,
                            "B" =>  true,
                            "B'" => true,
                            _ => false
                        }
                    }).count() as u8);
                    
                    tmp.extend(self.seq.split(" ").filter_map(|x| {
                        match x {
                            "U" =>  Some(0b1000),
                            "U'" => Some(0b0000),
                            "R" =>  Some(0b1001),
                            "R'" => Some(0b0001),
                            "F" =>  Some(0b1010),
                            "F'" => Some(0b0010),
                            "D" =>  Some(0b1011),
                            "D'" => Some(0b0011),
                            "L" =>  Some(0b1100),
                            "L'" => Some(0b0100),
                            "B" =>  Some(0b1101),
                            "B'" => Some(0b0101),
                            _ => None
                        }
                    }));
                    
                    for b in &tmp {
                        print!("0b{:b} ", b);
                    }
                    println!("");
                    self.send_via_port(&tmp.as_slice())
                })
                .color(color::rgb_bytes(0x00, 0x91, 0xEA))
                .set(SEQ_EXECUTE, ui);
        }
        
        { //Robot Status and options
            
        }
        
        { //Messages
            
        }
        
        { //Cube Input
            
            Canvas::new()
                .frame(0.0)
                .mid_top_of(CANVAS)
                .down_from(TITLEBAR, 16.0)
                .w(720.0)
                .h(592.0)
                .color(color::rgb_bytes(0xAB, 0x47, 0xBC))
                .set(FL_DIALOG, ui);
                
            Canvas::new()
                .frame(0.0)
                .w_of(FL_DIALOG)
                .h(48.0)
                .mid_top_of(FL_DIALOG)
                .color(color::rgb_bytes(0xE0, 0x40, 0xFB))
                .set(FL_TITLE_BAR, ui);
            
            Text::new("Cube Setup")
                .middle_of(FL_TITLE_BAR)
                .font_size(32)
                .color(color::rgb_bytes(0xFA, 0xFA, 0xFA))
                .set(FL_TITLE, ui);
                
            let side_name : [&str; 6]      = [               "U",                "R",                "F",                "D",                "L",                "B"];
            let side_pos : [(i32, i32); 6] = [/*U*/        (1,0), /*R*/        (2,1), /*F*/        (1,1), /*D*/        (1,2), /*L*/        (0,1), /*B*/        (3,1)];
            let colors : [(u8, u8, u8); 6] = [(0xFA, 0xFA, 0xFA), (0x29, 0x62, 0xFF), (0xD5, 0x00, 0x00), (0xFF, 0xFF, 0x00), (0x00, 0xC8, 0x53), (0xFF, 0x91, 0x00)]; /*Google Design A700 colors*/
            let bcolor : [(u8, u8, u8); 6] = [(0xE0, 0xE0, 0xE0), (0x1E, 0x88, 0xE5), (0xE5, 0x39, 0x35), (0xFD, 0xD8, 0x35), (0x43, 0xA0, 0x47), (0xFF, 0xB7, 0x4D)]; /*Google 600 colors for each*/
            
            for (i,&(x,y)) in side_pos.iter().enumerate() {
                Canvas::new()
                    .frame(0.0)
                    .w(160.0)
                    .h(160.0)
                    .mid_left_with_margin_on(FL_DIALOG, (16+172*x) as f64)
                    .down_from(FL_TITLE_BAR, (16+172*y) as f64)
                    .color(color::rgb_bytes(bcolor[i].0, bcolor[i].1, bcolor[i].2))
                    .set(FL_BOX + i, ui);
                
                let flp : [(i32, i32); 9] = [
                    (0, 0), (1, 0), (2, 0),
                    (0, 1), (1, 1), (2, 1),
                    (0, 2), (1, 2), (2, 2)
                ];
                
                for (j, &(x, y)) in flp.iter().enumerate() {
                    let flc = colors[self.facelets[9*i+j] as usize];
                    Button::new()
                        .frame(0.0)
                        .w_h(32.0, 32.0)
                        .label_font_size(12)
                        .label_color(color::rgb_bytes(0x10,0x10,0x10))
                        .label(&format!("{}{}", side_name[i], j+1))
                        .top_left_with_margins_on(FL_BOX + i, (16+48*y) as f64, (16+48*x) as f64)
                        .color(color::rgb_bytes(flc.0, flc.1, flc.2))
                        .react(||{
                            if j != 4 {
                                self.facelets[9*i+j] += 1;
                                self.facelets[9*i+j] %= 6;
                            }
                        })
                        .set(FL_FACELET + (9*i+j), ui);
                }
            }
            
            Button::new()
                .frame(0.0)
                .w_h(128.0, 64.0)
                .bottom_right_with_margin_on(FL_DIALOG, 16.0)
                .label_font_size(32)
                .label_color(color::rgb_bytes(0xff, 0xff, 0xff))
                .label("SOLVE!")
                .color(color::rgb_bytes(0xD5, 0x00, 0x00))
                .react(||{
                    let mut fl = [0u8; 54];
                    for (i,v) in self.facelets.iter().cloned().enumerate() {
                        fl[i] = match v {
                            0 => 'U' as u8,
                            1 => 'R' as u8,
                            2 => 'F' as u8,
                            3 => 'D' as u8,
                            4 => 'L' as u8,
                            5 => 'B' as u8,
                            _ => {panic!("Shouldn't be any other values'");}
                        };
                    }
                    let mut soln = min2phase::solution(&fl, 21, 100, 0, 0);
                    /*
                    let mut lmt = 1000;
                    while soln.starts_with("Error 8") && lmt >= 0 {
                        soln = min2phase::next(100, 0, 0);
                        lmt -= 1;
                    }// */
                    
                    println!("SOLUTION: '{}''", soln);
                    
                    let mut moves : Vec<u8> = vec![];
                    for x in soln.split(" ") {
                        println!("{}", x);
                        match x {
                            "U" =>  {moves.push(0b1000);},
                            "U'" => {moves.push(0b0000);},
                            "U2" => {moves.push(0b0000); moves.push(0b0000);},
                            "R" =>  {moves.push(0b1001);},
                            "R'" => {moves.push(0b0001);},
                            "R2" => {moves.push(0b0001); moves.push(0b0001);},
                            "F" =>  {moves.push(0b1010);},
                            "F'" => {moves.push(0b0010);},
                            "F2" => {moves.push(0b0010); moves.push(0b0010);},
                            "D" =>  {moves.push(0b1011);},
                            "D'" => {moves.push(0b0011);},
                            "D2" => {moves.push(0b0011); moves.push(0b0011);},
                            "L" =>  {moves.push(0b1100);},
                            "L'" => {moves.push(0b0100);},
                            "L2" => {moves.push(0b0100); moves.push(0b0100);},
                            "B" =>  {moves.push(0b1101);},
                            "B'" => {moves.push(0b0101);},
                            "B2" => {moves.push(0b0101); moves.push(0b0101);},
                            _ => ()
                        };
                    };
                    
                    let mut man = vec!['s' as u8, moves.len() as u8];
                    man.extend(moves.iter());
                    
                    println!("SENDING:");
                    for i in &man {
                        print!("{:b}  ", i);
                    }
                    println!("");
                    self.send_via_port(&man.as_slice());
                })
                .set(SOLVE, ui);
        }
    }
    
    fn set_port<'a>(&'a mut self, prt : &'a str) {
        
    }
}

static WDTH : u32 = 1600;
static HGHT : u32 = 900;

fn main() {
    unsafe {
        rs_2_java::rs_java_vm_create();
    }
    
    /*
    {
        let fl = [
            //U
            L, L, U,
            L, U, F,
            F, R, L,
            //R
        ];
        println!("{}", min2phase::solution());    
    }
    // */
    
    let opengl = OpenGL::V3_2;
    
    let mut window: PistonWindow = WindowSettings::new("Control", [WDTH, HGHT]).opengl(opengl).exit_on_esc(true).vsync(true).build().unwrap();
    
    let mut ui = {
        let theme = Theme::default();
        let glyph_cache = Glyphs::new("./font/NotoSans-Regular.ttf", window.factory.clone());
        Ui::new(glyph_cache.unwrap(), theme)
    };
    
    window.set_ups(60);
    
    let mut app = Control::new();
    
    min2phase::init();
    
    while let Some(event) = window.next() {
        ui.handle_event(&event);
        
        event.update(|_| ui.set_widgets(|mut ui| app.set_widgets(&mut ui)));
        
        window.draw_2d(&event, |c, g| ui.draw_if_changed(c, g));
    }
    
    unsafe {
        rs_2_java::rs_java_vm_destroy();
    }
}

#[allow(dead_code)]
widget_ids!{
    CANVAS,
    TITLEBAR,
    TITLE,
    
    //Port setter
    PORT_DIALOG,
    PORT_TITLE,
    PORT_TITLE_BAR,
    PORT_INPUT,
    SET_PORT,
    
    //Manual Control
    MANUAL_DIALOG,
    MANUAL_TITLE_BAR,
    MANUAL_TITLE,
    MANUAL_BIG with 6,
    MANUAL_SMALL with 6,
    MANUAL_BIG_PRIME with 6,
    MANUAL_SMALL_PRIME with 6,
    
    //Sequence Control
    SEQ_DIALOG,
    SEQ_TITLE_BAR,
    SEQ_TITLE,
    SEQ_TEXT_BOX,
    SEQ_EXECUTE,
    
    //Events and messages
    MSG_DIALOG,
    MSG_TITLE_BAR,
    MSG_TITLE,
    MSG_MSGS with 20,
    
    //Facelet cube inputs
    FL_DIALOG,
    FL_TITLE_BAR,
    FL_TITLE,
    FL_BOX with 6,
    FL_FACELET with 54,
    
    SOLVE
}