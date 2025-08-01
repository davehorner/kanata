//! Key code definitions.

/// Used for switch opcode purposes. Keys should not exceed this amount.
pub const KEY_MAX: u16 = 850;

#[test]
fn keycode_max_test() {
    assert!((KeyCode::KeyMax as u16) < KEY_MAX);
}

#[allow(missing_docs)]
/// Define a key code according to the HID specification.  Their names
/// correspond to the american QWERTY layout.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum KeyCode {
    ErrorUndefined = 0,
    Escape = 1,
    Kb1 = 2,
    Kb2 = 3,
    Kb3 = 4,
    Kb4 = 5,
    Kb5 = 6,
    Kb6 = 7,
    Kb7 = 8,
    Kb8 = 9,
    Kb9 = 10,
    Kb0 = 11,
    Minus = 12,
    Equal = 13,
    BSpace = 14,
    Tab = 15,
    Q = 16,
    W = 17,
    E = 18,
    R = 19,
    T = 20,
    Y = 21,
    U = 22,
    I = 23,
    O = 24,
    P = 25,
    LBracket = 26,
    RBracket = 27,
    Enter = 28,
    LCtrl = 29,
    A = 30,
    S = 31,
    D = 32,
    F = 33,
    G = 34,
    H = 35,
    J = 36,
    K = 37,
    L = 38,
    SColon = 39,
    Quote = 40,
    Grave = 41,
    LShift = 42,
    Bslash = 43,
    Z = 44,
    X = 45,
    C = 46,
    V = 47,
    B = 48,
    N = 49,
    M = 50,
    Comma = 51,
    Dot = 52,
    Slash = 53,
    RShift = 54,
    KpAsterisk = 55,
    LAlt = 56,
    Space = 57,
    CapsLock = 58,
    F1 = 59,
    F2 = 60,
    F3 = 61,
    F4 = 62,
    F5 = 63,
    F6 = 64,
    F7 = 65,
    F8 = 66,
    F9 = 67,
    F10 = 68,
    NumLock = 69,
    ScrollLock = 70,
    Kp7 = 71,
    Kp8 = 72,
    Kp9 = 73,
    KpMinus = 74,
    Kp4 = 75,
    Kp5 = 76,
    Kp6 = 77,
    KpPlus = 78,
    Kp1 = 79,
    Kp2 = 80,
    Kp3 = 81,
    Kp0 = 82,
    KpDot = 83,
    K0xBF = 84,
    K0xC0 = 85,
    NonUsBslash = 86,
    F11 = 87,
    F12 = 88,
    Intl1 = 89,
    K0xB1 = 90,
    K0xB3 = 91,
    K0xB0 = 92,
    K0xB2 = 93,
    K0xAF = 94,
    NonUsHash = 95,
    KpEnter = 96,
    RCtrl = 97,
    KpSlash = 98,
    SysReq = 99,
    RAlt = 100,
    K0xC1 = 101,
    Home = 102,
    Up = 103,
    PgUp = 104,
    Left = 105,
    Right = 106,
    End = 107,
    Down = 108,
    PgDown = 109,
    Insert = 110,
    Delete = 111,
    K0xC2 = 112,
    Mute = 113,
    VolDown = 114,
    VolUp = 115,
    Power = 116,
    KpEqual = 117,
    K0xC3 = 118,
    Pause = 119,
    K0xC4 = 120,
    KpComma = 121,
    Lang1 = 122,
    Lang2 = 123,
    Intl3 = 124,
    LGui = 125,
    RGui = 126,
    Application = 127,
    Stop = 128,
    Again = 129,
    K0xC5 = 130,
    Undo = 131,
    K0xC6 = 132,
    Copy = 133,
    K0xC7 = 134,
    Paste = 135,
    Find = 136,
    Cut = 137,
    Help = 138,
    Menu = 139,
    MediaCalc = 140,
    K0xC8 = 141,
    MediaSleep = 142,
    Wakeup = 143,
    K0xC9 = 144,
    K0xCA = 145,
    K0xCB = 146,
    K0xCC = 147,
    K0xCD = 148,
    K0xCE = 149,
    MediaWWW = 150,
    K0xCF = 151,
    MediaCoffee = 152,
    K0xD0 = 153,
    K0xD1 = 154,
    K0xAE = 155,
    K0xD2 = 156,
    K0xD3 = 157,
    MediaBack = 158,
    MediaForward = 159,
    MediaStop = 160,
    MediaEjectCD = 161,
    MediaFind = 162,
    MediaNextSong = 163,
    MediaPlayPause = 164,
    MediaPreviousSong = 165,
    MediaStopCD = 166,
    K0xD4 = 167,
    K0xD5 = 168,
    K0xD6 = 169,
    K0xD7 = 170,
    K0xD8 = 171,
    K0xAD = 172,
    MediaRefresh = 173,
    K0xD9 = 174,
    K0xDA = 175,
    MediaEdit = 176,
    MediaScrollUp = 177,
    MediaScrollDown = 178,
    K0xDB = 179,
    K0xDC = 180,
    K0xDD = 181,
    K0xDE = 182,
    F13 = 183,
    F14 = 184,
    F15 = 185,
    F16 = 186,
    F17 = 187,
    F18 = 188,
    F19 = 189,
    F20 = 190,
    F21 = 191,
    F22 = 192,
    F23 = 193,
    F24 = 194,
    Execute = 195,
    LockingCapsLock = 196,
    LockingNumLock = 197,
    LockingScrollLock = 198,
    KpEqualSign = 199,
    Intl2 = 200,
    Intl4 = 201,
    Intl5 = 202,
    Intl6 = 203,
    Intl7 = 204,
    Intl8 = 205,
    Intl9 = 206,
    Select = 207,
    Lang3 = 208,
    Lang4 = 209,
    PScreen = 210,
    Lang5 = 211,
    Lang6 = 212,
    Lang7 = 213,
    Lang8 = 214,
    K0xAB = 215,
    Lang9 = 216,
    K0xDF = 217,
    K0xBE = 218,
    Clear = 219,
    K220 = 220,
    K0xAC = 221,
    AltErase = 222,
    Cancel = 223,
    BrightnessDown = 224,
    BrightnessUp = 225,
    K0xAA = 226,
    Prior = 227,
    Return = 228,
    KbdIllumDown = 229,
    KbdIllumUp = 230,
    Separator = 231,
    Out = 232,
    Oper = 233,
    ClearAgain = 234,
    CrSel = 235,
    ExSel = 236,
    K0xB4 = 237,
    K0xB5 = 238,
    K0xB6 = 239,
    No = 240,
    K0xB7 = 241,
    K0xB8 = 242,
    K0xB9 = 243,
    K0xBA = 244,
    K0xBB = 245,
    K0xBC = 246,
    K0xBD = 247,
    MediaMute = 248,
    K249 = 249,
    PostFail = 250,
    ErrorRollOver = 251,
    K252 = 252,
    K253 = 253,
    K254 = 254,
    K255 = 255,
    K256 = 256,
    K257 = 257,
    K258 = 258,
    K259 = 259,
    K260 = 260,
    K261 = 261,
    K262 = 262,
    K263 = 263,
    K264 = 264,
    K265 = 265,
    K266 = 266,
    K267 = 267,
    K268 = 268,
    K269 = 269,
    K270 = 270,
    K271 = 271,
    K272 = 272,
    K273 = 273,
    K274 = 274,
    K275 = 275,
    K276 = 276,
    K277 = 277,
    K278 = 278,
    K279 = 279,
    K280 = 280,
    K281 = 281,
    K282 = 282,
    K283 = 283,
    K284 = 284,
    K285 = 285,
    K286 = 286,
    K287 = 287,
    K288 = 288,
    K289 = 289,
    K290 = 290,
    K291 = 291,
    K292 = 292,
    K293 = 293,
    K294 = 294,
    K295 = 295,
    K296 = 296,
    K297 = 297,
    K298 = 298,
    K299 = 299,
    K300 = 300,
    K301 = 301,
    K302 = 302,
    K303 = 303,
    K304 = 304,
    K305 = 305,
    K306 = 306,
    K307 = 307,
    K308 = 308,
    K309 = 309,
    K310 = 310,
    K311 = 311,
    K312 = 312,
    K313 = 313,
    K314 = 314,
    K315 = 315,
    K316 = 316,
    K317 = 317,
    K318 = 318,
    K319 = 319,
    K320 = 320,
    K321 = 321,
    K322 = 322,
    K323 = 323,
    K324 = 324,
    K325 = 325,
    K326 = 326,
    K327 = 327,
    K328 = 328,
    K329 = 329,
    K330 = 330,
    K331 = 331,
    K332 = 332,
    K333 = 333,
    K334 = 334,
    K335 = 335,
    K336 = 336,
    K337 = 337,
    K338 = 338,
    K339 = 339,
    K340 = 340,
    K341 = 341,
    K342 = 342,
    K343 = 343,
    K344 = 344,
    K345 = 345,
    K346 = 346,
    K347 = 347,
    K348 = 348,
    K349 = 349,
    K350 = 350,
    K351 = 351,
    K352 = 352,
    K353 = 353,
    K354 = 354,
    K355 = 355,
    K356 = 356,
    K357 = 357,
    K358 = 358,
    K359 = 359,
    K360 = 360,
    K361 = 361,
    K362 = 362,
    K363 = 363,
    K364 = 364,
    K365 = 365,
    K366 = 366,
    K367 = 367,
    K368 = 368,
    K369 = 369,
    K370 = 370,
    K371 = 371,
    K372 = 372,
    K373 = 373,
    K374 = 374,
    K375 = 375,
    K376 = 376,
    K377 = 377,
    K378 = 378,
    K379 = 379,
    K380 = 380,
    K381 = 381,
    K382 = 382,
    K383 = 383,
    K384 = 384,
    K385 = 385,
    K386 = 386,
    K387 = 387,
    K388 = 388,
    K389 = 389,
    K390 = 390,
    K391 = 391,
    K392 = 392,
    K393 = 393,
    K394 = 394,
    K395 = 395,
    K396 = 396,
    K397 = 397,
    K398 = 398,
    K399 = 399,
    K400 = 400,
    K401 = 401,
    K402 = 402,
    K403 = 403,
    K404 = 404,
    K405 = 405,
    K406 = 406,
    K407 = 407,
    K408 = 408,
    K409 = 409,
    K410 = 410,
    K411 = 411,
    K412 = 412,
    K413 = 413,
    K414 = 414,
    K415 = 415,
    K416 = 416,
    K417 = 417,
    K418 = 418,
    K419 = 419,
    K420 = 420,
    K421 = 421,
    K422 = 422,
    K423 = 423,
    K424 = 424,
    K425 = 425,
    K426 = 426,
    K427 = 427,
    K428 = 428,
    K429 = 429,
    K430 = 430,
    K431 = 431,
    K432 = 432,
    K433 = 433,
    K434 = 434,
    K435 = 435,
    K436 = 436,
    K437 = 437,
    K438 = 438,
    K439 = 439,
    K440 = 440,
    K441 = 441,
    K442 = 442,
    K443 = 443,
    K444 = 444,
    K445 = 445,
    K446 = 446,
    K447 = 447,
    K448 = 448,
    K449 = 449,
    K450 = 450,
    K451 = 451,
    K452 = 452,
    K453 = 453,
    K454 = 454,
    K455 = 455,
    K456 = 456,
    K457 = 457,
    K458 = 458,
    K459 = 459,
    K460 = 460,
    K461 = 461,
    K462 = 462,
    K463 = 463,
    K464 = 464,
    K465 = 465,
    K466 = 466,
    K467 = 467,
    K468 = 468,
    K469 = 469,
    K470 = 470,
    K471 = 471,
    K472 = 472,
    K473 = 473,
    K474 = 474,
    K475 = 475,
    K476 = 476,
    K477 = 477,
    K478 = 478,
    K479 = 479,
    K480 = 480,
    K481 = 481,
    K482 = 482,
    K483 = 483,
    K484 = 484,
    K485 = 485,
    K486 = 486,
    K487 = 487,
    K488 = 488,
    K489 = 489,
    K490 = 490,
    K491 = 491,
    K492 = 492,
    K493 = 493,
    K494 = 494,
    K495 = 495,
    K496 = 496,
    K497 = 497,
    K498 = 498,
    K499 = 499,
    K500 = 500,
    K501 = 501,
    K502 = 502,
    K503 = 503,
    K504 = 504,
    K505 = 505,
    K506 = 506,
    K507 = 507,
    K508 = 508,
    K509 = 509,
    K510 = 510,
    K511 = 511,
    K512 = 512,
    K513 = 513,
    K514 = 514,
    K515 = 515,
    K516 = 516,
    K517 = 517,
    K518 = 518,
    K519 = 519,
    K520 = 520,
    K521 = 521,
    K522 = 522,
    K523 = 523,
    K524 = 524,
    K525 = 525,
    K526 = 526,
    K527 = 527,
    K528 = 528,
    K529 = 529,
    K530 = 530,
    K531 = 531,
    K532 = 532,
    K533 = 533,
    K534 = 534,
    K535 = 535,
    K536 = 536,
    K537 = 537,
    K538 = 538,
    K539 = 539,
    K540 = 540,
    K541 = 541,
    K542 = 542,
    K543 = 543,
    K544 = 544,
    K545 = 545,
    K546 = 546,
    K547 = 547,
    K548 = 548,
    K549 = 549,
    K550 = 550,
    K551 = 551,
    K552 = 552,
    K553 = 553,
    K554 = 554,
    K555 = 555,
    K556 = 556,
    K557 = 557,
    K558 = 558,
    K559 = 559,
    K560 = 560,
    K561 = 561,
    K562 = 562,
    K563 = 563,
    K564 = 564,
    K565 = 565,
    K566 = 566,
    K567 = 567,
    K568 = 568,
    K569 = 569,
    K570 = 570,
    K571 = 571,
    K572 = 572,
    K573 = 573,
    K574 = 574,
    K575 = 575,
    K576 = 576,
    K577 = 577,
    K578 = 578,
    K579 = 579,
    K580 = 580,
    K581 = 581,
    K582 = 582,
    K583 = 583,
    K584 = 584,
    K585 = 585,
    K586 = 586,
    K587 = 587,
    K588 = 588,
    K589 = 589,
    K590 = 590,
    K591 = 591,
    K592 = 592,
    K593 = 593,
    K594 = 594,
    K595 = 595,
    K596 = 596,
    K597 = 597,
    K598 = 598,
    K599 = 599,
    K600 = 600,
    K601 = 601,
    K602 = 602,
    K603 = 603,
    K604 = 604,
    K605 = 605,
    K606 = 606,
    K607 = 607,
    K608 = 608,
    K609 = 609,
    K610 = 610,
    K611 = 611,
    K612 = 612,
    K613 = 613,
    K614 = 614,
    K615 = 615,
    K616 = 616,
    K617 = 617,
    K618 = 618,
    K619 = 619,
    K620 = 620,
    K621 = 621,
    K622 = 622,
    K623 = 623,
    K624 = 624,
    K625 = 625,
    K626 = 626,
    K627 = 627,
    K628 = 628,
    K629 = 629,
    K630 = 630,
    K631 = 631,
    K632 = 632,
    K633 = 633,
    K634 = 634,
    K635 = 635,
    K636 = 636,
    K637 = 637,
    K638 = 638,
    K639 = 639,
    K640 = 640,
    K641 = 641,
    K642 = 642,
    K643 = 643,
    K644 = 644,
    K645 = 645,
    K646 = 646,
    K647 = 647,
    K648 = 648,
    K649 = 649,
    K650 = 650,
    K651 = 651,
    K652 = 652,
    K653 = 653,
    K654 = 654,
    K655 = 655,
    K656 = 656,
    K657 = 657,
    K658 = 658,
    K659 = 659,
    K660 = 660,
    K661 = 661,
    K662 = 662,
    K663 = 663,
    K664 = 664,
    K665 = 665,
    K666 = 666,
    K667 = 667,
    K668 = 668,
    K669 = 669,
    K670 = 670,
    K671 = 671,
    K672 = 672,
    K673 = 673,
    K674 = 674,
    K675 = 675,
    K676 = 676,
    K677 = 677,
    K678 = 678,
    K679 = 679,
    K680 = 680,
    K681 = 681,
    K682 = 682,
    K683 = 683,
    K684 = 684,
    K685 = 685,
    K686 = 686,
    K687 = 687,
    K688 = 688,
    K689 = 689,
    K690 = 690,
    K691 = 691,
    K692 = 692,
    K693 = 693,
    K694 = 694,
    K695 = 695,
    K696 = 696,
    K697 = 697,
    K698 = 698,
    K699 = 699,
    K700 = 700,
    K701 = 701,
    K702 = 702,
    K703 = 703,
    K704 = 704,
    K705 = 705,
    K706 = 706,
    K707 = 707,
    K708 = 708,
    K709 = 709,
    K710 = 710,
    K711 = 711,
    K712 = 712,
    K713 = 713,
    K714 = 714,
    K715 = 715,
    K716 = 716,
    K717 = 717,
    K718 = 718,
    K719 = 719,
    K720 = 720,
    K721 = 721,
    K722 = 722,
    K723 = 723,
    K724 = 724,
    K725 = 725,
    K726 = 726,
    K727 = 727,
    K728 = 728,
    K729 = 729,
    K730 = 730,
    K731 = 731,
    K732 = 732,
    K733 = 733,
    K734 = 734,
    K735 = 735,
    K736 = 736,
    K737 = 737,
    K738 = 738,
    K739 = 739,
    K740 = 740,
    K741 = 741,
    K742 = 742,
    K743 = 743,
    K744 = 744,
    MWU = 745,
    MWD = 746,
    MWL = 747,
    MWR = 748,
    K749 = 749,
    K750 = 750,
    K751 = 751,
    K752 = 752,
    K753 = 753,
    K754 = 754,
    K755 = 755,
    K756 = 756,
    K757 = 757,
    K758 = 758,
    K759 = 759,
    K760 = 760,
    K761 = 761,
    K762 = 762,
    K763 = 763,
    K764 = 764,
    K765 = 765,
    K766 = 766,
    KeyMax = 767,
}

use core::fmt;
impl fmt::Display for KeyCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            KeyCode::Kb1 => write!(f, "1"),
            KeyCode::Kb2 => write!(f, "2"),
            KeyCode::Kb3 => write!(f, "3"),
            KeyCode::Kb4 => write!(f, "4"),
            KeyCode::Kb5 => write!(f, "5"),
            KeyCode::Kb6 => write!(f, "6"),
            KeyCode::Kb7 => write!(f, "7"),
            KeyCode::Kb8 => write!(f, "8"),
            KeyCode::Kb9 => write!(f, "9"),
            KeyCode::Kb0 => write!(f, "0"),
            KeyCode::LCtrl => write!(f, "‹⎈"),
            KeyCode::RCtrl => write!(f, "⎈›"),
            KeyCode::LShift => write!(f, "‹⇧"),
            KeyCode::RShift => write!(f, "⇧›"),
            KeyCode::LAlt => write!(f, "‹⎇"),
            KeyCode::RAlt => write!(f, "⎇›"),
            KeyCode::LGui => write!(f, "‹◆"),
            KeyCode::RGui => write!(f, "◆›"),
            KeyCode::Enter => write!(f, "⏎"),
            KeyCode::Escape => write!(f, "⎋"),
            KeyCode::BSpace => write!(f, "␈"),
            KeyCode::Tab => write!(f, "⭾"),
            KeyCode::Space => write!(f, "␠"),
            KeyCode::Minus => write!(f, "−"),
            KeyCode::Equal => write!(f, "="),
            KeyCode::LBracket => write!(f, "["),
            KeyCode::RBracket => write!(f, "]"),
            KeyCode::Bslash => write!(f, "\\"),
            KeyCode::NonUsHash => write!(f, "#"),
            KeyCode::SColon => write!(f, ";"),
            KeyCode::Quote => write!(f, "'"),
            KeyCode::Grave => write!(f, "`"),
            KeyCode::Comma => write!(f, ","),
            KeyCode::Dot => write!(f, "."),
            KeyCode::Slash => write!(f, "/"),
            KeyCode::CapsLock => write!(f, "⇪"),
            KeyCode::Insert => write!(f, "⎀"),
            KeyCode::Delete => write!(f, "␡"),
            KeyCode::Home => write!(f, "⇤"),
            KeyCode::End => write!(f, "⇥"),
            KeyCode::PgDown => write!(f, "⇟"),
            KeyCode::PgUp => write!(f, "⇞"),
            KeyCode::Down => write!(f, "▼"),
            KeyCode::Up => write!(f, "▲"),
            KeyCode::Right => write!(f, "▶"),
            KeyCode::Left => write!(f, "◀"),
            KeyCode::NumLock => write!(f, "⇭"),
            KeyCode::KpSlash => write!(f, "🔢/"),
            KeyCode::KpAsterisk => write!(f, "🔢*"),
            KeyCode::KpMinus => write!(f, "🔢−"),
            KeyCode::KpPlus => write!(f, "🔢+"),
            KeyCode::KpEnter => write!(f, "🔢⏎"),
            KeyCode::Kp0 => write!(f, "🔢0"),
            KeyCode::Kp1 => write!(f, "🔢1"),
            KeyCode::Kp2 => write!(f, "🔢2"),
            KeyCode::Kp3 => write!(f, "🔢3"),
            KeyCode::Kp4 => write!(f, "🔢4"),
            KeyCode::Kp5 => write!(f, "🔢5"),
            KeyCode::Kp6 => write!(f, "🔢6"),
            KeyCode::Kp7 => write!(f, "🔢7"),
            KeyCode::Kp8 => write!(f, "🔢8"),
            KeyCode::Kp9 => write!(f, "🔢9"),
            KeyCode::KpDot => write!(f, "🔢."),
            KeyCode::KpEqual => write!(f, "🔢="),
            KeyCode::NonUsBslash => write!(f, "|"),
            KeyCode::Application => write!(f, "☰"),
            KeyCode::Mute => write!(f, "🔇"),
            KeyCode::VolUp => write!(f, "🔊"),
            KeyCode::VolDown => write!(f, "🔉"),
            _ => write!(f, "{self:?}"),
        }
    }
}
