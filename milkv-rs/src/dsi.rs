pub const REG_SCL_TOP_BASE: u32 = 0x0A080000;
pub const REG_DSI_WRAP_BASE: u32 = 0x0300C000;

pub const REG_GOP_OFFSET: u32 = 0x800;

pub const REG_MAP_BASE: u32 = (REG_SCL_TOP_BASE + 0x1000);
pub const fn REG_SCL_IMG_BASE(x: u32) -> u32 { (REG_SCL_TOP_BASE + 0x2000 + 0x1000*x)}
pub const fn REG_SCL_CORE_BASE(x: u32) -> u32 {(REG_SCL_TOP_BASE + 0x4000 + 0x1000*x)}
pub const REG_SCL_DISP_BASE: u32 = (REG_SCL_TOP_BASE + 0x8000);
pub const REG_SCL_BT_BASE: u32 = (REG_SCL_TOP_BASE + 0x9000);
pub const REG_SCL_DSI_BASE: u32 = (REG_SCL_TOP_BASE + 0xA000);
pub const REG_SCL_CMDQ_BASE: u32 = (REG_SCL_TOP_BASE + 0xB000);

// ============== TOP ============== //
pub const REG_SCL_TOP_CFG0: u32 = (REG_SCL_TOP_BASE + 0x00);
pub const REG_SCL_TOP_CFG1: u32 = (REG_SCL_TOP_BASE + 0x04);
pub const REG_SCL_TOP_AXI: u32 = (REG_SCL_TOP_BASE + 0x08);
pub const REG_SCL_TOP_BT_CFG: u32 = (REG_SCL_TOP_BASE + 0x0C);
pub const REG_SCL_TOP_SHD: u32 = (REG_SCL_TOP_BASE + 0x10);
pub const REG_SCL_TOP_INTR_MASK: u32 = (REG_SCL_TOP_BASE + 0x30);
pub const REG_SCL_TOP_INTR_STATUS: u32 = (REG_SCL_TOP_BASE + 0x34);
pub const REG_SCL_TOP_INTR_ENABLE: u32 = (REG_SCL_TOP_BASE + 0x38);
pub const REG_SCL_TOP_IMG_CTRL: u32 = (REG_SCL_TOP_BASE + 0x40);
pub const REG_SCL_TOP_CMDQ_START: u32 = (REG_SCL_TOP_BASE + 0x44);
pub const REG_SCL_TOP_CMDQ_STOP: u32 = (REG_SCL_TOP_BASE + 0x48);
pub const REG_SCL_TOP_PG: u32 = (REG_SCL_TOP_BASE + 0x4C);
pub const REG_SCL_TOP_LVDSTX: u32 = (REG_SCL_TOP_BASE + 0x50);
pub const REG_SCL_TOP_BT_ENC: u32 = (REG_SCL_TOP_BASE + 0x60);
pub const REG_SCL_TOP_BT_SYNC_CODE: u32 = (REG_SCL_TOP_BASE + 0x64);
pub const REG_SCL_TOP_BT_BLK_DATA: u32 = (REG_SCL_TOP_BASE + 0x68);
pub const REG_SCL_TOP_VO_MUX: u32 = (REG_SCL_TOP_BASE + 0x70);

// ============== IMG ============== //
pub const fn REG_SCL_IMG_CFG(x: u32) -> u32 {(REG_SCL_IMG_BASE(x) + 0x00)}
pub const fn REG_SCL_IMG_OFFSET(x: u32) -> u32 {(REG_SCL_IMG_BASE(x) + 0x04)}
pub const fn REG_SCL_IMG_SIZE(x: u32) -> u32 {(REG_SCL_IMG_BASE(x) + 0x08)}
pub const fn REG_SCL_IMG_PITCH_Y(x: u32) -> u32 {(REG_SCL_IMG_BASE(x) + 0x0C)}
pub const fn REG_SCL_IMG_PITCH_C(x: u32) -> u32 {(REG_SCL_IMG_BASE(x) + 0x10)}
pub const fn REG_SCL_IMG_SHD(x: u32) -> u32 {(REG_SCL_IMG_BASE(x) + 0x14)}
pub const fn REG_SCL_IMG_ADDR0_L(x: u32) -> u32 {(REG_SCL_IMG_BASE(x) + 0x24)}
pub const fn REG_SCL_IMG_ADDR0_H(x: u32) -> u32 {(REG_SCL_IMG_BASE(x) + 0x28)}
pub const fn REG_SCL_IMG_ADDR1_L(x: u32) -> u32 {(REG_SCL_IMG_BASE(x) + 0x2C)}
pub const fn REG_SCL_IMG_ADDR1_H(x: u32) -> u32 {(REG_SCL_IMG_BASE(x) + 0x30)}
pub const fn REG_SCL_IMG_ADDR2_L(x: u32) -> u32 {(REG_SCL_IMG_BASE(x) + 0x34)}
pub const fn REG_SCL_IMG_ADDR2_H(x: u32) -> u32 {(REG_SCL_IMG_BASE(x) + 0x38)}
pub const fn REG_SCL_IMG_CSC_COEF0(x: u32) -> u32 {(REG_SCL_IMG_BASE(x) + 0x40)}
pub const fn REG_SCL_IMG_CSC_COEF1(x: u32) -> u32 {(REG_SCL_IMG_BASE(x) + 0x44)}
pub const fn REG_SCL_IMG_CSC_COEF2(x: u32) -> u32 {(REG_SCL_IMG_BASE(x) + 0x48)}
pub const fn REG_SCL_IMG_CSC_COEF3(x: u32) -> u32 {(REG_SCL_IMG_BASE(x) + 0x4C)}
pub const fn REG_SCL_IMG_CSC_COEF4(x: u32) -> u32 {(REG_SCL_IMG_BASE(x) + 0x50)}
pub const fn REG_SCL_IMG_CSC_COEF5(x: u32) -> u32 {(REG_SCL_IMG_BASE(x) + 0x54)}
pub const fn REG_SCL_IMG_CSC_SUB(x: u32) -> u32 {(REG_SCL_IMG_BASE(x) + 0x58)}
pub const fn REG_SCL_IMG_CSC_ADD(x: u32) -> u32 {(REG_SCL_IMG_BASE(x) + 0x5C)}
pub const fn REG_SCL_IMG_DBG(x: u32) -> u32 {(REG_SCL_IMG_BASE(x) + 0x68)}

// ============== SCL ============== //
pub const fn REG_SCL_CIR_BASE(x: u32) -> u32 {(REG_SCL_CORE_BASE(x) + 0x200)}
pub const fn REG_SCL_GOP_BASE(x: u32) -> u32 {(REG_SCL_CORE_BASE(x) + REG_GOP_OFFSET)}
pub const fn REG_SCL_BORDER_BASE(x: u32) -> u32 {(REG_SCL_CORE_BASE(x) + 0xA00)}
pub const fn REG_SCL_ODMA_BASE(x: u32) -> u32 {(REG_SCL_CORE_BASE(x) + 0xC00)}
pub const fn REG_SCL_CSC_BASE(x: u32) -> u32 {(REG_SCL_CORE_BASE(x) + 0xD00)}

// SCL
pub const fn REG_SCL_CFG(x: u32) -> u32 {(REG_SCL_CORE_BASE(x) + 0x00)}
pub const fn REG_SCL_SHD(x: u32) -> u32 {(REG_SCL_CORE_BASE(x) + 0x04)}
pub const fn REG_SCL_STATUS(x: u32) -> u32 {(REG_SCL_CORE_BASE(x) + 0x08)}
pub const fn REG_SCL_SRC_SIZE(x: u32) -> u32 {(REG_SCL_CORE_BASE(x) + 0x0c)}
pub const fn REG_SCL_CROP_OFFSET(x: u32) -> u32 {(REG_SCL_CORE_BASE(x) + 0x10)}
pub const fn REG_SCL_CROP_SIZE(x: u32) -> u32 {(REG_SCL_CORE_BASE(x) + 0x14)}
pub const fn REG_SCL_COEF0(x: u32) -> u32 {(REG_SCL_CORE_BASE(x) + 0x118)}
pub const fn REG_SCL_COEF1(x: u32) -> u32 {(REG_SCL_CORE_BASE(x) + 0x11C)}
pub const fn REG_SCL_COEF2(x: u32) -> u32 {(REG_SCL_CORE_BASE(x) + 0x120)}
pub const fn REG_SCL_COEF3(x: u32) -> u32 {(REG_SCL_CORE_BASE(x) + 0x124)}
pub const fn REG_SCL_COEF4(x: u32) -> u32 {(REG_SCL_CORE_BASE(x) + 0x128)}
pub const fn REG_SCL_COEF5(x: u32) -> u32 {(REG_SCL_CORE_BASE(x) + 0x12C)}
pub const fn REG_SCL_COEF6(x: u32) -> u32 {(REG_SCL_CORE_BASE(x) + 0x130)}
pub const fn REG_SCL_SC_CFG(x: u32) -> u32 {(REG_SCL_CORE_BASE(x) + 0x134)}
pub const fn REG_SCL_SC_H_CFG(x: u32) -> u32 {(REG_SCL_CORE_BASE(x) + 0x138)}
pub const fn REG_SCL_SC_V_CFG(x: u32) -> u32 {(REG_SCL_CORE_BASE(x) + 0x13C)}
pub const fn REG_SCL_OUT_SIZE(x: u32) -> u32 {(REG_SCL_CORE_BASE(x) + 0x140)}
pub const fn REG_SCL_SC_INI_PH(x: u32) -> u32 {(REG_SCL_CORE_BASE(x) + 0x148)}

// CIRCLE
pub const fn REG_SCL_CIR_CFG(x: u32) -> u32 {(REG_SCL_CIR_BASE(x) + 0x00)}
pub const fn REG_SCL_CIR_CENTER_X(x: u32) -> u32 {(REG_SCL_CIR_BASE(x) + 0x04)}
pub const fn REG_SCL_CIR_CENTER_Y(x: u32) -> u32 {(REG_SCL_CIR_BASE(x) + 0x08)}
pub const fn REG_SCL_CIR_RADIUS(x: u32) -> u32 {(REG_SCL_CIR_BASE(x) + 0x0C)}
pub const fn REG_SCL_CIR_SIZE(x: u32) -> u32 {(REG_SCL_CIR_BASE(x) + 0x10)}
pub const fn REG_SCL_CIR_OFFSET(x: u32) -> u32 {(REG_SCL_CIR_BASE(x) + 0x14)}
pub const fn REG_SCL_CIR_COLOR(x: u32) -> u32 {(REG_SCL_CIR_BASE(x) + 0x18)}

// GOP
pub const fn REG_SCL_GOP_FMT(x: u32, y: u32) -> u32 {(REG_SCL_GOP_BASE(x) + 0x20*y + 0x00)}
pub const fn REG_SCL_GOP_H_RANGE(x: u32, y: u32) -> u32 {(REG_SCL_GOP_BASE(x) + 0x20*y + 0x04)}
pub const fn REG_SCL_GOP_V_RANGE(x: u32, y: u32) -> u32 {(REG_SCL_GOP_BASE(x) + 0x20*y + 0x08)}
pub const fn REG_SCL_GOP_ADDR_L(x: u32, y: u32) -> u32 {(REG_SCL_GOP_BASE(x) + 0x20*y + 0x0c)}
pub const fn REG_SCL_GOP_ADDR_H(x: u32, y: u32) -> u32 {(REG_SCL_GOP_BASE(x) + 0x20*y + 0x10)}
pub const fn REG_SCL_GOP_PITCH(x: u32, y: u32) -> u32 {(REG_SCL_GOP_BASE(x) + 0x20*y + 0x14)}
pub const fn REG_SCL_GOP_SIZE(x: u32, y: u32) -> u32 {(REG_SCL_GOP_BASE(x) + 0x20*y + 0x18)}
pub const fn REG_SCL_GOP_CFG(x: u32) -> u32 {(REG_SCL_GOP_BASE(x) + 0x100)}
pub const fn REG_SCL_GOP_LUT0(x: u32) -> u32 {(REG_SCL_GOP_BASE(x) + 0x104)}
pub const fn REG_SCL_GOP_LUT1(x: u32) -> u32 {(REG_SCL_GOP_BASE(x) + 0x108)}
pub const fn REG_SCL_GOP_COLORKEY(x: u32) -> u32 {(REG_SCL_GOP_BASE(x) + 0x10c)}
pub const fn REG_SCL_GOP_FONTCOLOR(x: u32) -> u32 {(REG_SCL_GOP_BASE(x) + 0x110)}

// BORDER
pub const fn REG_SCL_BORDER_CFG(x: u32) -> u32 {(REG_SCL_BORDER_BASE(x) + 0x00)}
pub const fn REG_SCL_BORDER_OFFSET(x: u32) -> u32 {(REG_SCL_BORDER_BASE(x) + 0x04)}

// ODMA
pub const fn REG_SCL_ODMA_CFG(x: u32) -> u32 {(REG_SCL_ODMA_BASE(x) + 0x00)}
pub const fn REG_SCL_ODMA_ADDR0_L(x: u32) -> u32 {(REG_SCL_ODMA_BASE(x) + 0x04)}
pub const fn REG_SCL_ODMA_ADDR0_H(x: u32) -> u32 {(REG_SCL_ODMA_BASE(x) + 0x08)}
pub const fn REG_SCL_ODMA_ADDR1_L(x: u32) -> u32 {(REG_SCL_ODMA_BASE(x) + 0x0C)}
pub const fn REG_SCL_ODMA_ADDR1_H(x: u32) -> u32 {(REG_SCL_ODMA_BASE(x) + 0x10)}
pub const fn REG_SCL_ODMA_ADDR2_L(x: u32) -> u32 {(REG_SCL_ODMA_BASE(x) + 0x14)}
pub const fn REG_SCL_ODMA_ADDR2_H(x: u32) -> u32 {(REG_SCL_ODMA_BASE(x) + 0x18)}
pub const fn REG_SCL_ODMA_PITCH_Y(x: u32) -> u32 {(REG_SCL_ODMA_BASE(x) + 0x1C)}
pub const fn REG_SCL_ODMA_PITCH_C(x: u32) -> u32 {(REG_SCL_ODMA_BASE(x) + 0x20)}
pub const fn REG_SCL_ODMA_OFFSET_X(x: u32) -> u32 {(REG_SCL_ODMA_BASE(x) + 0x24)}
pub const fn REG_SCL_ODMA_OFFSET_Y(x: u32) -> u32 {(REG_SCL_ODMA_BASE(x) + 0x28)}
pub const fn REG_SCL_ODMA_WIDTH(x: u32) -> u32 {(REG_SCL_ODMA_BASE(x) + 0x2C)}
pub const fn REG_SCL_ODMA_HEIGHT(x: u32) -> u32 {(REG_SCL_ODMA_BASE(x) + 0x30)}
pub const fn REG_SCL_ODMA_DBG(x: u32) -> u32 {(REG_SCL_ODMA_BASE(x) + 0x34)}

// CSC
pub const fn REG_SCL_CSC_EN(x: u32) -> u32 {(REG_SCL_CSC_BASE(x) + 0x00)}
pub const fn REG_SCL_CSC_COEF0(x: u32) -> u32 {(REG_SCL_CSC_BASE(x) + 0x04)}
pub const fn REG_SCL_CSC_COEF1(x: u32) -> u32 {(REG_SCL_CSC_BASE(x) + 0x08)}
pub const fn REG_SCL_CSC_COEF2(x: u32) -> u32 {(REG_SCL_CSC_BASE(x) + 0x0c)}
pub const fn REG_SCL_CSC_COEF3(x: u32) -> u32 {(REG_SCL_CSC_BASE(x) + 0x10)}
pub const fn REG_SCL_CSC_COEF4(x: u32) -> u32 {(REG_SCL_CSC_BASE(x) + 0x14)}
pub const fn REG_SCL_CSC_OFFSET(x: u32) -> u32 {(REG_SCL_CSC_BASE(x) + 0x18)}
pub const fn REG_SCL_CSC_FRAC0(x: u32) -> u32 {(REG_SCL_CSC_BASE(x) + 0x1C)}
pub const fn REG_SCL_CSC_FRAC1(x: u32) -> u32 {(REG_SCL_CSC_BASE(x) + 0x20)}

// ============== DISP ============== //
pub const REG_SCL_DISP_CFG: u32 = (REG_SCL_DISP_BASE + 0x00);
pub const REG_SCL_DISP_TOTAL: u32 = (REG_SCL_DISP_BASE + 0x04);
pub const REG_SCL_DISP_VSYNC: u32 = (REG_SCL_DISP_BASE + 0x08);
pub const REG_SCL_DISP_VFDE: u32 = (REG_SCL_DISP_BASE + 0x0C);
pub const REG_SCL_DISP_VMDE: u32 = (REG_SCL_DISP_BASE + 0x10);
pub const REG_SCL_DISP_HSYNC: u32 = (REG_SCL_DISP_BASE + 0x14);
pub const REG_SCL_DISP_HFDE: u32 = (REG_SCL_DISP_BASE + 0x18);
pub const REG_SCL_DISP_HMDE: u32 = (REG_SCL_DISP_BASE + 0x1C);
pub const REG_SCL_DISP_ADDR0_L: u32 = (REG_SCL_DISP_BASE + 0x34);
pub const REG_SCL_DISP_ADDR0_H: u32 = (REG_SCL_DISP_BASE + 0x38);
pub const REG_SCL_DISP_ADDR1_L: u32 = (REG_SCL_DISP_BASE + 0x3C);
pub const REG_SCL_DISP_ADDR1_H: u32 = (REG_SCL_DISP_BASE + 0x40);
pub const REG_SCL_DISP_ADDR2_L: u32 = (REG_SCL_DISP_BASE + 0x44);
pub const REG_SCL_DISP_ADDR2_H: u32 = (REG_SCL_DISP_BASE + 0x48);
pub const REG_SCL_DISP_PITCH_Y: u32 = (REG_SCL_DISP_BASE + 0x4C);
pub const REG_SCL_DISP_PITCH_C: u32 = (REG_SCL_DISP_BASE + 0x50);
pub const REG_SCL_DISP_OFFSET: u32 = (REG_SCL_DISP_BASE + 0x50);
pub const REG_SCL_DISP_SIZE: u32 = (REG_SCL_DISP_BASE + 0x58);
pub const REG_SCL_DISP_OUT_CSC0: u32 = (REG_SCL_DISP_BASE + 0x5C);
pub const REG_SCL_DISP_OUT_CSC1: u32 = (REG_SCL_DISP_BASE + 0x60);
pub const REG_SCL_DISP_OUT_CSC2: u32 = (REG_SCL_DISP_BASE + 0x64);
pub const REG_SCL_DISP_OUT_CSC3: u32 = (REG_SCL_DISP_BASE + 0x68);
pub const REG_SCL_DISP_OUT_CSC4: u32 = (REG_SCL_DISP_BASE + 0x6C);
pub const REG_SCL_DISP_OUT_CSC_SUB: u32 = (REG_SCL_DISP_BASE + 0x70);
pub const REG_SCL_DISP_OUT_CSC_ADD: u32 = (REG_SCL_DISP_BASE + 0x74);
pub const REG_SCL_DISP_IN_CSC0: u32 = (REG_SCL_DISP_BASE + 0x78);
pub const REG_SCL_DISP_IN_CSC1: u32 = (REG_SCL_DISP_BASE + 0x7C);
pub const REG_SCL_DISP_IN_CSC2: u32 = (REG_SCL_DISP_BASE + 0x80);
pub const REG_SCL_DISP_IN_CSC3: u32 = (REG_SCL_DISP_BASE + 0x84);
pub const REG_SCL_DISP_IN_CSC4: u32 = (REG_SCL_DISP_BASE + 0x88);
pub const REG_SCL_DISP_IN_CSC_SUB: u32 = (REG_SCL_DISP_BASE + 0x8C);
pub const REG_SCL_DISP_IN_CSC_ADD: u32 = (REG_SCL_DISP_BASE + 0x90);
pub const REG_SCL_DISP_PAT_CFG: u32 = (REG_SCL_DISP_BASE + 0x94);
pub const REG_SCL_DISP_PAT_COLOR0: u32 = (REG_SCL_DISP_BASE + 0x98);
pub const REG_SCL_DISP_PAT_COLOR1: u32 = (REG_SCL_DISP_BASE + 0x9C);
pub const REG_SCL_DISP_PAT_COLOR2: u32 = (REG_SCL_DISP_BASE + 0xA0);
pub const REG_SCL_DISP_PAT_COLOR3: u32 = (REG_SCL_DISP_BASE + 0xA4);
pub const REG_SCL_DISP_PAT_COLOR4: u32 = (REG_SCL_DISP_BASE + 0xA8);
pub const REG_SCL_DISP_DBG: u32 = (REG_SCL_DISP_BASE + 0xAC);
pub const REG_SCL_DISP_AXI_ST: u32 = (REG_SCL_DISP_BASE + 0xB0);
pub const REG_SCL_DISP_CACHE: u32 = (REG_SCL_DISP_BASE + 0xC0);
pub const REG_SCL_DISP_DUMMY: u32 = (REG_SCL_DISP_BASE + 0xF8);

// GAMMA
pub const REG_SCL_DISP_GAMMA_CTRL: u32 = (REG_SCL_DISP_BASE + 0x180);
pub const REG_SCL_DISP_GAMMA_WR_LUT: u32 = (REG_SCL_DISP_BASE + 0x184);
pub const REG_SCL_DISP_GAMMA_RD_LUT: u32 = (REG_SCL_DISP_BASE + 0x188);

// i80
pub const REG_SCL_DISP_MCU_IF_CTRL: u32 = (REG_SCL_DISP_BASE + 0x200);
pub const REG_SCL_DISP_MCU_SW_CTRL: u32 = (REG_SCL_DISP_BASE + 0x204);
pub const REG_SCL_DISP_MCU_STATUS: u32 = (REG_SCL_DISP_BASE + 0x208);

// GOP
pub const fn REG_SCL_DISP_GOP_FMT(y: u32) -> u32 {(REG_SCL_DISP_BASE + REG_GOP_OFFSET + 0x20*y + 0x00)}
pub const fn REG_SCL_DISP_GOP_H_RANGE(y: u32) -> u32 {(REG_SCL_DISP_BASE + REG_GOP_OFFSET + 0x20*y + 0x04)}
pub const fn REG_SCL_DISP_GOP_V_RANGE(y: u32) -> u32 {(REG_SCL_DISP_BASE + REG_GOP_OFFSET + 0x20*y + 0x08)}
pub const fn REG_SCL_DISP_GOP_ADDR_L(y: u32) -> u32 {(REG_SCL_DISP_BASE + REG_GOP_OFFSET + 0x20*y + 0x0c)}
pub const fn REG_SCL_DISP_GOP_ADDR_H(y: u32) -> u32 {(REG_SCL_DISP_BASE + REG_GOP_OFFSET + 0x20*y + 0x10)}
pub const fn REG_SCL_DISP_GOP_PITCH(y: u32) -> u32 {(REG_SCL_DISP_BASE + REG_GOP_OFFSET + 0x20*y + 0x14)}
pub const fn REG_SCL_DISP_GOP_SIZE(y: u32) -> u32 {(REG_SCL_DISP_BASE + REG_GOP_OFFSET + 0x20*y + 0x18)}
pub const REG_SCL_DISP_GOP_CFG: u32 = (REG_SCL_DISP_BASE + REG_GOP_OFFSET + 0x100);
pub const REG_SCL_DISP_GOP_LUT0: u32 = (REG_SCL_DISP_BASE + REG_GOP_OFFSET + 0x104);
pub const REG_SCL_DISP_GOP_LUT1: u32 = (REG_SCL_DISP_BASE + REG_GOP_OFFSET + 0x108);
pub const REG_SCL_DISP_GOP_COLORKEY: u32 = (REG_SCL_DISP_BASE + REG_GOP_OFFSET + 0x10c);
pub const REG_SCL_DISP_GOP_FONTCOLOR: u32 = (REG_SCL_DISP_BASE + REG_GOP_OFFSET + 0x110);

// ============== ROT ============== //
pub const REG_SCL_ROT_CFG: u32 = (REG_SCL_ROT_BASE + 0x00);
pub const REG_SCL_ROT_SHD: u32 = (REG_SCL_ROT_BASE + 0x04);
pub const REG_SCL_ROT_IDLE: u32 = (REG_SCL_ROT_BASE + 0x08);
pub const REG_SCL_ROT_FMT: u32 = (REG_SCL_ROT_BASE + 0x0C);
pub const REG_SCL_ROT_SRC_ADDR_L: u32 = (REG_SCL_ROT_BASE + 0x10);
pub const REG_SCL_ROT_SRC_ADDR_H: u32 = (REG_SCL_ROT_BASE + 0x14);
pub const REG_SCL_ROT_DST_ADDR_L: u32 = (REG_SCL_ROT_BASE + 0x18);
pub const REG_SCL_ROT_DST_ADDR_H: u32 = (REG_SCL_ROT_BASE + 0x1C);
pub const REG_SCL_ROT_SRC_PITCH: u32 = (REG_SCL_ROT_BASE + 0x20);
pub const REG_SCL_ROT_DST_PITCH: u32 = (REG_SCL_ROT_BASE + 0x24);
pub const REG_SCL_ROT_SRC_OFFSET_X: u32 = (REG_SCL_ROT_BASE + 0x28);
pub const REG_SCL_ROT_SRC_OFFSET_Y: u32 = (REG_SCL_ROT_BASE + 0x2C);
pub const REG_SCL_ROT_DST_OFFSET_X: u32 = (REG_SCL_ROT_BASE + 0x30);
pub const REG_SCL_ROT_DST_OFFSET_Y: u32 = (REG_SCL_ROT_BASE + 0x34);
pub const REG_SCL_ROT_WIDTH: u32 = (REG_SCL_ROT_BASE + 0x38);
pub const REG_SCL_ROT_HEIGHT: u32 = (REG_SCL_ROT_BASE + 0x3C);

// ============== DSI ============== //
pub const REG_SCL_DSI_MAC_EN: u32 = (REG_SCL_DSI_BASE + 0x00);
pub const REG_SCL_DSI_HS_0: u32 = (REG_SCL_DSI_BASE + 0x04);
pub const REG_SCL_DSI_HS_1: u32 = (REG_SCL_DSI_BASE + 0x08);
pub const REG_SCL_DSI_ESC: u32 = (REG_SCL_DSI_BASE + 0x0C);
pub const REG_SCL_DSI_ESC_TX0: u32 = (REG_SCL_DSI_BASE + 0x10);
pub const REG_SCL_DSI_ESC_TX1: u32 = (REG_SCL_DSI_BASE + 0x14);
pub const REG_SCL_DSI_ESC_TX2: u32 = (REG_SCL_DSI_BASE + 0x18);
pub const REG_SCL_DSI_ESC_TX3: u32 = (REG_SCL_DSI_BASE + 0x1C);
pub const REG_SCL_DSI_ESC_RX0: u32 = (REG_SCL_DSI_BASE + 0x20);
pub const REG_SCL_DSI_ESC_RX1: u32 = (REG_SCL_DSI_BASE + 0x24);

// ============== DSI PHY ============== //
pub const REG_DSI_PHY_EN: u32 = (REG_DSI_WRAP_BASE + 0x00);
pub const REG_DSI_PHY_CLK_CFG1: u32 = (REG_DSI_WRAP_BASE + 0x04);
pub const REG_DSI_PHY_CLK_CFG2: u32 = (REG_DSI_WRAP_BASE + 0x08);
pub const REG_DSI_PHY_ESC_INIT: u32 = (REG_DSI_WRAP_BASE + 0x0C);
pub const REG_DSI_PHY_ESC_WAKE: u32 = (REG_DSI_WRAP_BASE + 0x10);
pub const REG_DSI_PHY_HS_CFG1: u32 = (REG_DSI_WRAP_BASE + 0x14);
pub const REG_DSI_PHY_HS_CFG2: u32 = (REG_DSI_WRAP_BASE + 0x18);
pub const REG_DSI_PHY_CAL_CFG: u32 = (REG_DSI_WRAP_BASE + 0x1C);
pub const REG_DSI_PHY_CAL_NUM: u32 = (REG_DSI_WRAP_BASE + 0x20);
pub const REG_DSI_PHY_CLK_STATE: u32 = (REG_DSI_WRAP_BASE + 0x24);
pub const REG_DSI_PHY_DATA0_STATE: u32 = (REG_DSI_WRAP_BASE + 0x28);
pub const REG_DSI_PHY_DATA12_STATE: u32 = (REG_DSI_WRAP_BASE + 0x2C);
pub const REG_DSI_PHY_DATA3_STATE: u32 = (REG_DSI_WRAP_BASE + 0x30);
pub const REG_DSI_PHY_HS_OV: u32 = (REG_DSI_WRAP_BASE + 0x38);
pub const REG_DSI_PHY_HS_SW1: u32 = (REG_DSI_WRAP_BASE + 0x3C);
pub const REG_DSI_PHY_HS_SW2: u32 = (REG_DSI_WRAP_BASE + 0x40);
pub const REG_DSI_PHY_DATA_OV: u32 = (REG_DSI_WRAP_BASE + 0x44);
pub const REG_DSI_PHY_LPTX_OV: u32 = (REG_DSI_WRAP_BASE + 0x4C);
pub const REG_DSI_PHY_LPRX_OV: u32 = (REG_DSI_WRAP_BASE + 0x4C);
pub const REG_DSI_PHY_PD: u32 = (REG_DSI_WRAP_BASE + 0x64);
pub const REG_DSI_PHY_TXPLL: u32 = (REG_DSI_WRAP_BASE + 0x6C);
pub const REG_DSI_PHY_REG_8C: u32 = (REG_DSI_WRAP_BASE + 0x8C);
pub const REG_DSI_PHY_REG_SET: u32 = (REG_DSI_WRAP_BASE + 0x90);
pub const REG_DSI_PHY_LANE_SEL: u32 = (REG_DSI_WRAP_BASE + 0x9C);
pub const REG_DSI_PHY_LANE_PN_SWAP: u32 = (REG_DSI_WRAP_BASE + 0xA0);
pub const REG_DSI_PHY_LVDS_EN: u32 = (REG_DSI_WRAP_BASE + 0xB4);
pub const REG_DSI_PHY_EXT_GPIO: u32 = (REG_DSI_WRAP_BASE + 0xC0);