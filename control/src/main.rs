#[macro_use] extern crate conrod;
extern crate piston_window;
extern crate serial;

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

fn encode_seq(ms : &[u8]) -> Vec<u8> {
    let mut r :Vec<u8> = vec!['s' as u8,ms.len() as u8];
    r.extend(ms.iter().cloned());
    r
}

struct Control {
    port : Option<Box<SerialPort>>,
    port_name : String,
}

impl Control {
    fn new() -> Control {
        Control {
            port : None,
            port_name : "".to_string(),
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
                .color(color::rgb_bytes(0x01, 0x57, 0x9B))
                .scroll_kids()
                .set(CANVAS, ui);
                
            Canvas::new()
                .frame(0.0)
                .w_of(CANVAS)
                .h(64.0)
                .mid_top_of(CANVAS)
                .color(color::rgb_bytes(0x29, 0xB6, 0xF6))
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
                .color(color::rgb_bytes(0x02, 0x77, 0xBD))
                .set(MANUAL_DIALOG, ui);
                
            Canvas::new()
                .frame(0.0)
                .w_of(MANUAL_DIALOG)
                .h(48.0)
                .mid_top_of(MANUAL_DIALOG)
                .color(color::rgb_bytes(0x00, 0xB0, 0xFF))
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
                    .color(color::rgb_bytes(0x00, 0x91, 0xEA))
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
                    .color(color::rgb_bytes(0x00, 0x91, 0xEA))
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
                    .color(color::rgb_bytes(0x00, 0x91, 0xEA))
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
                    .color(color::rgb_bytes(0x00, 0x91, 0xEA))
                    .react(|| {
                        println!("SEND: {}+", sidi[i]);
                        self.send_via_port(&[sidi[i] as u8 ^ (1<<5), '-' as u8]);
                    })
                    .set(MANUAL_SMALL_PRIME + i, ui);
            }
            
        }
    }
    
    fn set_port<'a>(&'a mut self, prt : &'a str) {
        
    }
}

static WDTH : u32 = 1280;
static HGHT : u32 = 960;

fn main() {
    let opengl = OpenGL::V3_2;
    
    let mut window: PistonWindow = WindowSettings::new("Control", [WDTH, HGHT]).opengl(opengl).exit_on_esc(true).vsync(true).build().unwrap();
    
    let mut ui = {
        let theme = Theme::default();
        let glyph_cache = Glyphs::new("./font/NotoSans-Regular.ttf", window.factory.clone());
        Ui::new(glyph_cache.unwrap(), theme)
    };
    
    window.set_ups(60);
    
    let mut app = Control::new();
    
    while let Some(event) = window.next() {
        ui.handle_event(&event);
        
        event.update(|_| ui.set_widgets(|mut ui| app.set_widgets(&mut ui)));
        
        window.draw_2d(&event, |c, g| ui.draw_if_changed(c, g));
    }
}

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
    SEQ_EXECUTE
}