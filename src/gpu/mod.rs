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
        }
    }

    /// Handle writes to the GP0 command register
    pub fn gp0(&mut self, val: u32) {
        let opcode = (val >> 24) & 0xff;

        match opcode {
            0xe1 => self.gp0_draw_mode(val),
            _ => panic!("Unhandled GP0 command 0x{:08x}", val),
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
}