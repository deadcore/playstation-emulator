use self::displaydepth::DisplayDepth;
use self::dmadirection::DmaDirection;
use self::field::Field;
use self::resolution::{HorizontalRes, VerticalRes};
use self::texturedepth::TextureDepth;
use self::vmode::VMode;

pub mod texturedepth;
pub mod field;
pub mod resolution;
pub mod vmode;
pub mod displaydepth;
pub mod dmadirection;

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
}

impl Gpu {
    pub fn new() -> Gpu {
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
        }
    }

    /// Handle writes to the GP0 command register
    pub fn gp0(&mut self, val: u32) {
        let opcode = (val >> 24) & 0xff;

        match opcode {
            0xe1 => self.gp0_draw_mode(val),
            0x00 => (), // NOP
            _ => panic!("Unhandled GP0 command 0x{:08x}", val),
        }
    }

    pub fn gp1(&mut self, val: u32) {
        let opcode = (val >> 24) & 0xff;

        match opcode {
            0x00 => self.gp1_reset(),
            0x08 => self.gp1_display_mode(val),
            _ => panic!("Unhandled GP1 command 0x{:08x}", val),
        }
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
    fn gp0_draw_mode(&mut self, val: u32) {
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
    }
}