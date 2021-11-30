pub const NOP: u8 = 0;
pub const ACONST_NULL: u8 = 1;
pub const ICONST_M1: u8 = 2;
pub const ICONST_0: u8 = 3;
pub const ICONST_1: u8 = 4;
pub const ICONST_2: u8 = 5;
pub const ICONST_3: u8 = 6;
pub const ICONST_4: u8 = 7;
pub const ICONST_5: u8 = 8;
pub const LCONST_0: u8 = 9;
pub const LCONST_1: u8 = 10;
pub const FCONST_0: u8 = 11;
pub const FCONST_1: u8 = 12;
pub const FCONST_2: u8 = 13;
pub const DCONST_0: u8 = 14;
pub const DCONST_1: u8 = 15;
pub const BIPUSH: u8 = 16;
pub const SIPUSH: u8 = 17;
pub const LDC: u8 = 18;
pub const ILOAD: u8 = 21;
pub const LLOAD: u8 = 22;
pub const FLOAD: u8 = 23;
pub const DLOAD: u8 = 24;
pub const ALOAD: u8 = 25;
pub const IALOAD: u8 = 46;
pub const LALOAD: u8 = 47;
pub const FALOAD: u8 = 48;
pub const DALOAD: u8 = 49;
pub const AALOAD: u8 = 50;
pub const BALOAD: u8 = 51;
pub const CALOAD: u8 = 52;
pub const SALOAD: u8 = 53;
pub const ISTORE: u8 = 54;
pub const LSTORE: u8 = 55;
pub const FSTORE: u8 = 56;
pub const DSTORE: u8 = 57;
pub const ASTORE: u8 = 58;
pub const IASTORE: u8 = 79;
pub const LASTORE: u8 = 80;
pub const FASTORE: u8 = 81;
pub const DASTORE: u8 = 82;
pub const AASTORE: u8 = 83;
pub const BASTORE: u8 = 84;
pub const CASTORE: u8 = 85;
pub const SASTORE: u8 = 86;
pub const POP: u8 = 87;
pub const POP2: u8 = 88;
pub const DUP: u8 = 89;
pub const DUP_X1: u8 = 90;
pub const DUP_X2: u8 = 91;
pub const DUP2: u8 = 92;
pub const DUP2_X1: u8 = 93;
pub const DUP2_X2: u8 = 94;
pub const SWAP: u8 = 95;
pub const IADD: u8 = 96;
pub const LADD: u8 = 97;
pub const FADD: u8 = 98;
pub const DADD: u8 = 99;
pub const ISUB: u8 = 100;
pub const LSUB: u8 = 101;
pub const FSUB: u8 = 102;
pub const DSUB: u8 = 103;
pub const IMUL: u8 = 104;
pub const LMUL: u8 = 105;
pub const FMUL: u8 = 106;
pub const DMUL: u8 = 107;
pub const IDIV: u8 = 108;
pub const LDIV: u8 = 109;
pub const FDIV: u8 = 110;
pub const DDIV: u8 = 111;
pub const IREM: u8 = 112;
pub const LREM: u8 = 113;
pub const FREM: u8 = 114;
pub const DREM: u8 = 115;
pub const INEG: u8 = 116;
pub const LNEG: u8 = 117;
pub const FNEG: u8 = 118;
pub const DNEG: u8 = 119;
pub const ISHL: u8 = 120;
pub const LSHL: u8 = 121;
pub const ISHR: u8 = 122;
pub const LSHR: u8 = 123;
pub const IUSHR: u8 = 124;
pub const LUSHR: u8 = 125;
pub const IAND: u8 = 126;
pub const LAND: u8 = 127;
pub const IOR: u8 = 128;
pub const LOR: u8 = 129;
pub const IXOR: u8 = 130;
pub const LXOR: u8 = 131;
pub const IINC: u8 = 132;
pub const I2L: u8 = 133;
pub const I2F: u8 = 134;
pub const I2D: u8 = 135;
pub const L2I: u8 = 136;
pub const L2F: u8 = 137;
pub const L2D: u8 = 138;
pub const F2I: u8 = 139;
pub const F2L: u8 = 140;
pub const F2D: u8 = 141;
pub const D2I: u8 = 142;
pub const D2L: u8 = 143;
pub const D2F: u8 = 144;
pub const I2B: u8 = 145;
pub const I2C: u8 = 146;
pub const I2S: u8 = 147;
pub const LCMP: u8 = 148;
pub const FCMPL: u8 = 149;
pub const FCMPG: u8 = 150;
pub const DCMPL: u8 = 151;
pub const DCMPG: u8 = 152;
pub const IFEQ: u8 = 153;
pub const IFNE: u8 = 154;
pub const IFLT: u8 = 155;
pub const IFGE: u8 = 156;
pub const IFGT: u8 = 157;
pub const IFLE: u8 = 158;
pub const IF_ICMPEQ: u8 = 159;
pub const IF_ICMPNE: u8 = 160;
pub const IF_ICMPLT: u8 = 161;
pub const IF_ICMPGE: u8 = 162;
pub const IF_ICMPGT: u8 = 163;
pub const IF_ICMPLE: u8 = 164;
pub const IF_ACMPEQ: u8 = 165;
pub const IF_ACMPNE: u8 = 166;
pub const GOTO: u8 = 167;
pub const JSR: u8 = 168;
pub const RET: u8 = 169;
pub const TABLESWITCH: u8 = 170;
pub const LOOKUPSWITCH: u8 = 171;
pub const IRETURN: u8 = 172;
pub const LRETURN: u8 = 173;
pub const FRETURN: u8 = 174;
pub const DRETURN: u8 = 175;
pub const ARETURN: u8 = 176;
pub const RETURN: u8 = 177;
pub const GETSTATIC: u8 = 178;
pub const PUTSTATIC: u8 = 179;
pub const GETFIELD: u8 = 180;
pub const PUTFIELD: u8 = 181;
pub const INVOKEVIRTUAL: u8 = 182;
pub const INVOKESPECIAL: u8 = 183;
pub const INVOKESTATIC: u8 = 184;
pub const INVOKEINTERFACE: u8 = 185;
pub const INVOKEDYNAMIC: u8 = 186;
pub const NEW: u8 = 187;
pub const NEWARRAY: u8 = 188;
pub const ANEWARRAY: u8 = 189;
pub const ARRAYLENGTH: u8 = 190;
pub const ATHROW: u8 = 191;
pub const CHECKCAST: u8 = 192;
pub const INSTANCEOF: u8 = 193;
pub const MONITORENTER: u8 = 194;
pub const MONITOREXIT: u8 = 195;
pub const MULTIANEWARRAY: u8 = 197;
pub const IFNULL: u8 = 198;
pub const IFNONNULL: u8 = 199;