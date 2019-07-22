use std::fmt;

use crate::gpu::commandbuffer::CommandBuffer;
use crate::gpu::opengl::{Color, Renderer, Vertex};
use crate::gpu::opengl::Position;

use self::displaydepth::DisplayDepth;
use self::dmadirection::DmaDirection;
use self::field::Field;
use self::resolution::{HorizontalRes, VerticalRes};
use self::texturedepth::TextureDepth;
use self::vmode::VMode;

pub mod opengl;

pub mod texturedepth;
pub mod field;
pub mod resolution;
pub mod vmode;
pub mod displaydepth;
pub mod dmadirection;
pub mod commandbuffer;

pub struct Gpu {
    /// Texture page base X coordinate (4 bits , 64 byte increment )
    page_base_x: u8,

    /// Texture page base Y coordinate (1bit , 256 line increment)
    page_base_y: u8,

    /// Semi−transparency. Not entirely sure how to handle that value
    /// yet, it seems to describe how to blend the source and
    /// destination colors .
    semi_transparency: u8,

    /// Texture page color depth
    texture_depth: TextureDepth,

    /// Enable dithering from 24 to 15bits RGB
    dithering: bool,

    /// Allow drawing to the display area
    draw_to_display: bool,

    /// Force ”mask” bit of the pixel to 1 when writing to VRAM /// (otherwise don’t modify it)
    force_set_mask_bit: bool,

    /// Don't draw to pixels which have the "mask" bit set
    preserve_masked_pixels: bool,

    /// Don’t draw to pixels which have the ”mask” bit set preserve masked pixels : bool ,
    /// Currently displayed field. For progressive output this is /// always Top.
    field: Field,

    /// When true all textures are disabled
    texture_disable: bool,

    /// Video output horizontal resolution
    hres: HorizontalRes,

    /// Video output vertical resolution
    vres: VerticalRes,

    /// Video mode
    vmode: VMode,

    /// Display depth. The GPU itself always draws 15bit RGB, 24bit
    /// output must use external assets (pre−rendered textures , MDEC, etc...)
    display_depth: DisplayDepth,

    /// Output interlaced video signal instead of progressive
    interlaced: bool,

    /// Disable the display
    display_disabled: bool,

    /// True when the interrupt is active
    interrupt: bool,

    /// DMA request direction
    dma_direction: DmaDirection,

    rectangle_texture_x_flip: bool,

    rectangle_texture_y_flip: bool,

    /// Texture window x mask (8 pixel steps)
    texture_window_x_mask: u8,

    /// Texture window y mask (8 pixel steps)
    texture_window_y_mask: u8,

    /// Texture window x offset (8 pixel steps)
    texture_window_x_offset: u8,

    /// Texture window y offset (8 pixel steps)
    texture_window_y_offset: u8,

    /// Left-most column of drawing area
    drawing_area_left: u16,

    /// Top-most line of drawing area
    drawing_area_top: u16,

    /// Right-most column of drawing area
    drawing_area_right: u16,

    /// Bottom-most line of drawing area
    drawing_area_bottom: u16,

    /// Horizontal drawing offset applied to all vertex
    drawing_x_offset: i16,
    /// Vertical drawing offset applied to all vertex
    drawing_y_offset: i16,

    /// First column of the display area in VRAM
    display_vram_x_start: u16,

    /// First line of the display area in VRAM
    display_vram_y_start: u16,

    /// Display output horizontal start relative to HSYNC
    display_horiz_start: u16,

    /// Display output horizontal end relative to HSYNC
    display_horiz_end: u16,

    /// Display output first line relative to VSYNC
    display_line_start: u16,

    /// Display output last line relative to VSYNC
    display_line_end: u16,

    /// Buffer containing the current GP0 command
    gp0_command: CommandBuffer,
    /// Remaining words for the current GP0 command
    gp0_words_remaining: u32,
    /// Pointer to the method implementing the current GP) command
    gp0_command_method: fn(&mut Gpu),

    /// Current mode of the GP0 register
    gp0_mode: Gp0Mode,

    /// OpenGL renderer
    renderer: Renderer,
}

impl Gpu {
    pub fn new(renderer: Renderer) -> Gpu {
        Gpu {
            page_base_x: 0,
            page_base_y: 0,
            semi_transparency: 0,
            texture_depth: TextureDepth::T4Bit,
            dithering: false,
            draw_to_display: false,
            force_set_mask_bit: false,
            preserve_masked_pixels: false,
            field: Field::Top,
            texture_disable: false,
            hres: HorizontalRes::from_fields(0, 0),
            vres: VerticalRes::Y240Lines,
            vmode: VMode::Ntsc,
            display_depth: DisplayDepth::D15Bits,
            interlaced: false,
            display_disabled: true,
            interrupt: false,
            dma_direction: DmaDirection::Off,
            rectangle_texture_x_flip: false,
            rectangle_texture_y_flip: false,
            texture_window_x_mask: 0,
            texture_window_y_mask: 0,
            texture_window_x_offset: 0,
            texture_window_y_offset: 0,
            drawing_area_left: 0,
            drawing_area_top: 0,
            drawing_area_right: 0,
            drawing_area_bottom: 0,
            drawing_x_offset: 0,
            drawing_y_offset: 0,
            display_vram_x_start: 0,
            display_vram_y_start: 0,
            display_horiz_start: 0,
            display_horiz_end: 0,
            display_line_start: 0,
            display_line_end: 0,
            gp0_command: CommandBuffer::new(),
            gp0_words_remaining: 0,
            gp0_command_method: Gpu::gp0_nop,
            gp0_mode: Gp0Mode::Command,
            renderer,
        }
    }

    /// Handle writes to the GP0 command register
    pub fn gp0(&mut self, val: u32) {
        if self.gp0_words_remaining == 0 {
            // We start a new GP0 command
            let opcode = (val >> 24) & 0xff;
            debug!("GP0 execution - 0x{:08x} with opcode: [0x{:02x}]", val, opcode);

            let (len, method): (u32, fn(&mut Gpu)) = match opcode {
                0x00 => (1, Gpu::gp0_nop),
                0x01 => (1, Gpu::gp0_clear_cache),
                0xe1 => (1, Gpu::gp0_draw_mode),
                0xa0 => (3, Gpu::gp0_image_load),
                0xc0 => (3, Gpu::gp0_image_store),
                0x28 => (5, Gpu::gp0_quad_mono_opaque),
                0x38 => (8, Gpu::gp0_quad_shaded_opaque),
                0x30 => (6, Gpu::gp0_triangle_shaded_opaque),
                0x2c => (9, Gpu::gp0_quad_texture_blend_opaque),
                0xe2 => (1, Gpu::gp0_texture_window),
                0xe3 => (1, Gpu::gp0_drawing_area_top_left),
                0xe4 => (1, Gpu::gp0_drawing_area_bottom_right),
                0xe5 => (1, Gpu::gp0_drawing_offset),
                0xe6 => (1, Gpu::gp0_mask_bit_setting),
                _ => panic!("Unhandled GP0 command 0x{:08x}", val),
            };

            self.gp0_words_remaining = len;
            self.gp0_command_method = method;
            self.gp0_command.clear();
        }

        self.gp0_words_remaining -= 1;

        match self.gp0_mode {
            Gp0Mode::Command => {
                self.gp0_command.push_word(val);

                if self.gp0_words_remaining == 0 {
                    // We have all the parameters, we can run the command
                    (self.gp0_command_method)(self);
                }
            }
            Gp0Mode::ImageLoad => {
                // XXX Should copy pixel data to VRAM
                if self.gp0_words_remaining == 0 {
                    // Load done, switch back to command mode
                    self.gp0_mode = Gp0Mode::Command;
                }
            }
        }
    }

    pub fn gp1(&mut self, val: u32) {
        let opcode = (val >> 24) & 0xff;

        debug!("GP1 execution - 0x{:08x} with opcode: [0x{:02x}]", val, opcode);

        match opcode {
            0x00 => self.gp1_reset(),
            0x08 => self.gp1_display_mode(val),
            0x04 => self.gp1_dma_direction(val),
            0x05 => self.gp1_display_vram_start(val),
            0x06 => self.gp1_display_horizontal_range(val),
            0x07 => self.gp1_display_vertical_range(val),
            0x03 => self.gp1_display_enable(val),
            0x02 => self.gp1_acknowledge_irg(),
            0x01 => self.gp1_reset_command_buffer(),
            _ => panic!("Unhandled GP1 command 0x{:08x}", val),
        }
    }

    /// GP0(0x01) : Clear Cache
    fn gp0_clear_cache(&mut self) {
        // Not implemented
        warn!("GP0(0x01): Clear Cache - Not Implemented")
    }

    fn gp0_nop(&mut self) {}

    /// GP0(0x38): Shaded Opaque Quadrilateral
    fn gp0_quad_shaded_opaque(&mut self) {
        let vertices = [
            Vertex::new(Position::from_packed(self.gp0_command[1]), Color::from_packed(self.gp0_command[0])),
            Vertex::new(Position::from_packed(self.gp0_command[3]), Color::from_packed(self.gp0_command[2])),
            Vertex::new(Position::from_packed(self.gp0_command[5]), Color::from_packed(self.gp0_command[4])),
            Vertex::new(Position::from_packed(self.gp0_command[7]), Color::from_packed(self.gp0_command[6])),
        ];

        self.renderer.push_quad(&vertices);
    }

    /// GP0(0x30) : Shaded Opaque Triangle
    fn gp0_triangle_shaded_opaque(&mut self) {
        let vertices = [
            Vertex::new(Position::from_packed(self.gp0_command[1]), Color::from_packed(self.gp0_command[0])),
            Vertex::new(Position::from_packed(self.gp0_command[3]), Color::from_packed(self.gp0_command[2])),
            Vertex::new(Position::from_packed(self.gp0_command[5]), Color::from_packed(self.gp0_command[4])),
        ];

        self.renderer.push_triangle(&vertices);
    }

    /// GP0(0x2c): Textured Opaque Quadrilateral
    fn gp0_quad_texture_blend_opaque(&mut self) {
        warn!("[Unhandled] 0x2c - Draw quad texture blending");
    }

    /// GP0(0XA0): Image Load
    fn gp0_image_load(&mut self) {
        // Parameter 2 contains the image resolution
        let res = self.gp0_command[2];
        let width = res & 0xffff;
        let height = res >> 16;

        // Size of the image in 16bit pixels
        let imgsize = width * height;

        // If we have an odd number of pixels we must round up
        // since we transfer 32bits at a time. There’ll be 16bits
        // of padding in the last word.
        let imgsize = (imgsize + 1) & !1;
        // Store number of words expected for this image
        self.gp0_words_remaining = imgsize / 2;

        // Put the GP0 state machine in ImageLoad mode
        self.gp0_mode = Gp0Mode::ImageLoad;
    }

    /// GP0(0xC0): Image Store
    fn gp0_image_store(&mut self) {
        // Parameter 2 contains the image resolution
        let res = self.gp0_command[2];
        let width = res & 0xffff;
        let height = res >> 16;
        warn!("Unhandled image store: {}x{}", width, height);
    }

    /// GP0(0xE2): Set Texture Window
    fn gp0_texture_window(&mut self) {
        let val = self.gp0_command[0];

        self.texture_window_x_mask = (val & 0x1f) as u8;
        self.texture_window_y_mask = ((val >> 5) & 0x1f) as u8;
        self.texture_window_x_offset = ((val >> 10) & 0x1f) as u8;
        self.texture_window_y_offset = ((val >> 15) & 0x1f) as u8;
    }

    /// GP0(0xE3): Set Drawing Area top left
    fn gp0_drawing_area_top_left(&mut self) {
        let val = self.gp0_command[0];

        self.drawing_area_top = ((val >> 10) & 0x3ff) as u16;
        self.drawing_area_left = (val & 0x3ff) as u16;
        self.update_drawing_area();
    }

    /// GP0(0xE4): Set Drawing Area bottom right
    fn gp0_drawing_area_bottom_right(&mut self) {
        let val = self.gp0_command[0];

        self.drawing_area_bottom = ((val >> 10) & 0x3ff) as u16;
        self.drawing_area_right = (val & 0x3ff) as u16;

        self.update_drawing_area();
    }

    // Called when the drawing area changes to notify the renderer
    fn update_drawing_area(&mut self) {
        self.renderer.set_drawing_area(self.drawing_area_left,
                                       self.drawing_area_top,
                                       self.drawing_area_right,
                                       self.drawing_area_bottom);
    }

    /// GP0(0xE5): Set Drawing Offset
    fn gp0_drawing_offset(&mut self) {
        let val = self.gp0_command[0];

        let x = (val & 0x7ff) as u16;
        let y = ((val >> 11) & 0x7ff) as u16;

        // Values are 11bit two's complement signed values, we need to
        // shift the value to 16bits to force sign extension
        let x = ((x << 5) as i16) >> 5;
        let y = ((y << 5) as i16) >> 5;

        // Values are 11bit two's complement signed values, we need to
        // shift the value to 16bits to force sign extension
        self.drawing_x_offset = x;
        self.drawing_y_offset = y;


        self.renderer.set_draw_offset(x, y);
    }

    /// GP0(0xE6): Set Mask Bit Setting
    fn gp0_mask_bit_setting(&mut self) {
        let val = self.gp0_command[0];

        self.force_set_mask_bit = (val & 1) != 0;
        self.preserve_masked_pixels = (val & 2) != 0;
    }

    /// GP0(0x28): Monochrome Opaque Quadrilateral
    fn gp0_quad_mono_opaque(&mut self) {
        // Only one color repeated 4 times
        let color = Color::from_packed(self.gp0_command[0]);

        let vertices = [
            Vertex::new(Position::from_packed(self.gp0_command[1]), color),
            Vertex::new(Position::from_packed(self.gp0_command[2]), color),
            Vertex::new(Position::from_packed(self.gp0_command[3]), color),
            Vertex::new(Position::from_packed(self.gp0_command[4]), color),
        ];

        self.renderer.push_quad(&vertices);
    }

    /// GP1(0x01): Reset Command Buffer
    fn gp1_reset_command_buffer(&mut self) {
        self.gp0_command.clear();
        self.gp0_words_remaining = 0;
        self.gp0_mode = Gp0Mode::Command;
        // XXX should also clear the command FIFO when we implement it
    }

    /// GP1(0x02) Acknowledge Interrupt
    fn gp1_acknowledge_irg(&mut self) {
        self.interrupt = false;
    }

    /// GP1(0x03) : Display Enable
    fn gp1_display_enable(&mut self, val: u32) {
        self.display_disabled = val & 1 != 0;
    }

    /// GP1(0x07) : Display Vertical Range
    fn gp1_display_vertical_range(&mut self, val: u32) {
        self.display_line_start = (val & 0x3ff) as u16;
        self.display_line_end = ((val >> 10) & 0x3ff) as u16;
    }

    /// GP1(0x06) : Display Horizontal Range
    fn gp1_display_horizontal_range(&mut self, val: u32) {
        self.display_horiz_start = (val & 0xfff) as u16;
        self.display_horiz_end = ((val >> 12) & 0xfff) as u16;
    }

    /// GP1(0x05) : Display VRAM Start
    fn gp1_display_vram_start(&mut self, val: u32) {
        self.display_vram_x_start = (val & 0x3fe) as u16;
        self.display_vram_y_start = ((val >> 10) & 0x1ff) as u16;
    }

    fn gp1_dma_direction(&mut self, val: u32) {
        self.dma_direction = match val & 3 {
            0 => DmaDirection::Off,
            1 => DmaDirection::Fifo,
            2 => DmaDirection::CpuToGp0,
            3 => DmaDirection::VRamToCpu,
            _ => unreachable!(),
        };
    }

    /// GP1(0x80): Display Mode
    fn gp1_display_mode(&mut self, val: u32) {
        let hr1 = (val & 3) as u8;
        let hr2 = ((val >> 6) & 1) as u8;

        self.hres = HorizontalRes::from_fields(hr1, hr2);

        self.vres = match val & 0x4 != 0 {
            false => VerticalRes::Y240Lines,
            true => VerticalRes::Y480Lines,
        };

        self.vmode = match val & 0x8 != 0 {
            false => VMode::Ntsc,
            true => VMode::Pal,
        };

        self.display_depth = match val & 0x10 != 0 {
            false => DisplayDepth::D24Bits,
            true => DisplayDepth::D15Bits,
        };

        self.interlaced = val & 0x20 != 0;

        if val & 0x80 != 0 {
            panic!("Unsupported display mode 0x{:08x}", val);
        }
    }

    /// GP0(0xE1) command
    fn gp0_draw_mode(&mut self) {
        let val = self.gp0_command[0];

        self.page_base_x = (val & 0xf) as u8;
        self.page_base_y = ((val >> 4) & 1) as u8;
        self.semi_transparency = ((val >> 5) & 3) as u8;

        self.texture_depth = match (val >> 7) & 3 {
            0 => TextureDepth::T4Bit,
            1 => TextureDepth::T8Bit,
            2 => TextureDepth::T15Bit,
            n => panic!("Unhandled texture depth {}", n)
        };

        self.dithering = ((val >> 9) & 1) != 0;
        self.draw_to_display = ((val >> 10) & 1) != 0;
        self.texture_disable = ((val >> 11) & 1) != 0;
        self.rectangle_texture_x_flip = ((val >> 12) & 1) != 0;
        self.rectangle_texture_y_flip = ((val >> 13) & 1) != 0;
    }

    /// Retrieve value of the status register
    fn status(&self) -> u32 {
        let mut r = 0u32;

        r |= (self.page_base_x as u32) << 0;
        r |= (self.page_base_y as u32) << 4;
        r |= (self.semi_transparency as u32) << 5;
        r |= (self.texture_depth as u32) << 7;
        r |= (self.dithering as u32) << 9;
        r |= (self.draw_to_display as u32) << 10;
        r |= (self.force_set_mask_bit as u32) << 11;
        r |= (self.preserve_masked_pixels as u32) << 12;
        r |= (self.field as u32) << 13;
        // Bit 14: not supported
        r |= (self.texture_disable as u32) << 15;
        r |= self.hres.into_status();
        r |= (self.vres as u32) << 19;
        r |= (self.vmode as u32) << 20;
        r |= (self.display_depth as u32) << 21;
        r |= (self.interlaced as u32) << 22;
        r |= (self.display_disabled as u32) << 23;
        r |= (self.interrupt as u32) << 24;

        // For now we pretend that the GPU is always ready:
        // Ready to receive command
        r |= 1 << 26;
        // Ready to send VRAM to CPU
        r |= 1 << 27;
        // Ready to receive DMA block
        r |= 1 << 28;

        r |= (self.dma_direction as u32) << 29;

        // Bit 31 should change depending on the currently drawn
        // line (whether it’s even, odd or in the vblack
        // apparently). Let’s not bother with it for now.
        r |= 0 << 31;

        // Not sure about that, I'm guessing that it's the signal
        // checked by the DMA in when sending data in Request
        // synchronization mode. For now I blindly follow the Nocash
        // spec.
        let dma_request =
            match self.dma_direction {
                // Always 0
                DmaDirection::Off => 0,
                // Should be 0 if FIFO is full, 1 otherwise
                DmaDirection::Fifo => 1,
                // Should be the same as status bit 28
                DmaDirection::CpuToGp0 => (r >> 28) & 1,
                // Should be the same as status bit 27
                DmaDirection::VRamToCpu => (r >> 27) & 1,
            };

        r |= dma_request << 25;

        r
    }

    /// GP1(0x00): Soft Reset
    fn gp1_reset(&mut self) {
        self.interrupt = false;
        self.page_base_x = 0;
        self.page_base_y = 0;
        self.semi_transparency = 0;
        self.texture_depth = TextureDepth::T4Bit;
        self.texture_window_x_mask = 0;
        self.texture_window_y_mask = 0;
        self.texture_window_x_offset = 0;
        self.texture_window_y_offset = 0;
        self.dithering = false;
        self.draw_to_display = false;
        self.texture_disable = false;
        self.rectangle_texture_x_flip;
        self.rectangle_texture_y_flip;
        self.drawing_area_left = 0;
        self.drawing_area_top = 0;
        self.drawing_area_right = 0;
        self.drawing_area_bottom = 0;
        self.drawing_x_offset = 0;
        self.drawing_y_offset = 0;
        self.force_set_mask_bit = false;
        self.preserve_masked_pixels = false;
        self.dma_direction = DmaDirection::Off;
        self.display_disabled = true;
        self.display_vram_x_start = 0;
        self.display_vram_y_start = 0;
        self.hres = HorizontalRes::from_fields(0, 0);
        self.vres = VerticalRes::Y240Lines;
        self.vmode = VMode::Ntsc;
        self.interlaced = true;
        self.display_horiz_start = 0x200;
        self.display_horiz_end = 0xc00;
        self.display_line_start = 0x10;
        self.display_line_end = 0x100;
        self.display_depth = DisplayDepth::D15Bits;

        self.renderer.set_draw_offset(0, 0);
    }

    /// Parse a position as written in the GP0 register and return it as
    /// an array of two `i16`
    fn gp0_position(pos: u32) -> [i16; 2] {
        let x = pos as i16;
        let y = (pos >> 16) as i16;

        [x, y]
    }

    fn gp0_color(col: u32) -> [u8; 3] {
        let r = col as u8;
        let g = (col >> 8) as u8;
        let b = (col >> 16) as u8;

        [r, g, b]
    }
}

/// Possible states for the GP0 command register
enum Gp0Mode {
    /// Default mode: handling commands
    Command,
    /// Loading an image into VRAM
    ImageLoad,
}

impl fmt::Display for Gp0Mode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            Gp0Mode::Command => "Command",
            Gp0Mode::ImageLoad => "Command",
        };
        write!(f, "{:?}", name)
    }
}