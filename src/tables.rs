// DO NOT EDIT THIS FILE. IT WAS AUTOMATICALLY GENERATED BY:
//
//   yeslogic-ucd-generate script --rust-enum --name Script ../ucd-generate/ucd-15.0.0
//
// Unicode version: 15.0.0.
//
// yeslogic-ucd-generate 0.6.0 is available on crates.io.

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum Script {
    Unknown,
    Adlam,
    Ahom,
    AnatolianHieroglyphs,
    Arabic,
    Armenian,
    Avestan,
    Balinese,
    Bamum,
    BassaVah,
    Batak,
    Bengali,
    Bhaiksuki,
    Bopomofo,
    Brahmi,
    Braille,
    Buginese,
    Buhid,
    CanadianAboriginal,
    Carian,
    CaucasianAlbanian,
    Chakma,
    Cham,
    Cherokee,
    Chorasmian,
    Common,
    Coptic,
    Cuneiform,
    Cypriot,
    CyproMinoan,
    Cyrillic,
    Deseret,
    Devanagari,
    DivesAkuru,
    Dogra,
    Duployan,
    EgyptianHieroglyphs,
    Elbasan,
    Elymaic,
    Ethiopic,
    Georgian,
    Glagolitic,
    Gothic,
    Grantha,
    Greek,
    Gujarati,
    GunjalaGondi,
    Gurmukhi,
    Han,
    Hangul,
    HanifiRohingya,
    Hanunoo,
    Hatran,
    Hebrew,
    Hiragana,
    ImperialAramaic,
    Inherited,
    InscriptionalPahlavi,
    InscriptionalParthian,
    Javanese,
    Kaithi,
    Kannada,
    Katakana,
    Kawi,
    KayahLi,
    Kharoshthi,
    KhitanSmallScript,
    Khmer,
    Khojki,
    Khudawadi,
    Lao,
    Latin,
    Lepcha,
    Limbu,
    LinearA,
    LinearB,
    Lisu,
    Lycian,
    Lydian,
    Mahajani,
    Makasar,
    Malayalam,
    Mandaic,
    Manichaean,
    Marchen,
    MasaramGondi,
    Medefaidrin,
    MeeteiMayek,
    MendeKikakui,
    MeroiticCursive,
    MeroiticHieroglyphs,
    Miao,
    Modi,
    Mongolian,
    Mro,
    Multani,
    Myanmar,
    Nabataean,
    NagMundari,
    Nandinagari,
    NewTaiLue,
    Newa,
    Nko,
    Nushu,
    NyiakengPuachueHmong,
    Ogham,
    OlChiki,
    OldHungarian,
    OldItalic,
    OldNorthArabian,
    OldPermic,
    OldPersian,
    OldSogdian,
    OldSouthArabian,
    OldTurkic,
    OldUyghur,
    Oriya,
    Osage,
    Osmanya,
    PahawhHmong,
    Palmyrene,
    PauCinHau,
    PhagsPa,
    Phoenician,
    PsalterPahlavi,
    Rejang,
    Runic,
    Samaritan,
    Saurashtra,
    Sharada,
    Shavian,
    Siddham,
    Signwriting,
    Sinhala,
    Sogdian,
    SoraSompeng,
    Soyombo,
    Sundanese,
    SylotiNagri,
    Syriac,
    Tagalog,
    Tagbanwa,
    TaiLe,
    TaiTham,
    TaiViet,
    Takri,
    Tamil,
    Tangsa,
    Tangut,
    Telugu,
    Thaana,
    Thai,
    Tibetan,
    Tifinagh,
    Tirhuta,
    Toto,
    Ugaritic,
    Vai,
    Vithkuqi,
    Wancho,
    WarangCiti,
    Yezidi,
    Yi,
    ZanabazarSquare,
}

pub const SCRIPT: &'static [(u32, u32, Script)] = &[
    (0, 64, Script::Common),
    (65, 90, Script::Latin),
    (91, 96, Script::Common),
    (97, 122, Script::Latin),
    (123, 169, Script::Common),
    (170, 170, Script::Latin),
    (171, 185, Script::Common),
    (186, 186, Script::Latin),
    (187, 191, Script::Common),
    (192, 214, Script::Latin),
    (215, 215, Script::Common),
    (216, 246, Script::Latin),
    (247, 247, Script::Common),
    (248, 696, Script::Latin),
    (697, 735, Script::Common),
    (736, 740, Script::Latin),
    (741, 745, Script::Common),
    (746, 747, Script::Bopomofo),
    (748, 767, Script::Common),
    (768, 879, Script::Inherited),
    (880, 883, Script::Greek),
    (884, 884, Script::Common),
    (885, 887, Script::Greek),
    (890, 893, Script::Greek),
    (894, 894, Script::Common),
    (895, 895, Script::Greek),
    (900, 900, Script::Greek),
    (901, 901, Script::Common),
    (902, 902, Script::Greek),
    (903, 903, Script::Common),
    (904, 906, Script::Greek),
    (908, 908, Script::Greek),
    (910, 929, Script::Greek),
    (931, 993, Script::Greek),
    (994, 1007, Script::Coptic),
    (1008, 1023, Script::Greek),
    (1024, 1156, Script::Cyrillic),
    (1157, 1158, Script::Inherited),
    (1159, 1327, Script::Cyrillic),
    (1329, 1366, Script::Armenian),
    (1369, 1418, Script::Armenian),
    (1421, 1423, Script::Armenian),
    (1425, 1479, Script::Hebrew),
    (1488, 1514, Script::Hebrew),
    (1519, 1524, Script::Hebrew),
    (1536, 1540, Script::Arabic),
    (1541, 1541, Script::Common),
    (1542, 1547, Script::Arabic),
    (1548, 1548, Script::Common),
    (1549, 1562, Script::Arabic),
    (1563, 1563, Script::Common),
    (1564, 1566, Script::Arabic),
    (1567, 1567, Script::Common),
    (1568, 1599, Script::Arabic),
    (1600, 1600, Script::Common),
    (1601, 1610, Script::Arabic),
    (1611, 1621, Script::Inherited),
    (1622, 1647, Script::Arabic),
    (1648, 1648, Script::Inherited),
    (1649, 1756, Script::Arabic),
    (1757, 1757, Script::Common),
    (1758, 1791, Script::Arabic),
    (1792, 1805, Script::Syriac),
    (1807, 1866, Script::Syriac),
    (1869, 1871, Script::Syriac),
    (1872, 1919, Script::Arabic),
    (1920, 1969, Script::Thaana),
    (1984, 2042, Script::Nko),
    (2045, 2047, Script::Nko),
    (2048, 2093, Script::Samaritan),
    (2096, 2110, Script::Samaritan),
    (2112, 2139, Script::Mandaic),
    (2142, 2142, Script::Mandaic),
    (2144, 2154, Script::Syriac),
    (2160, 2190, Script::Arabic),
    (2192, 2193, Script::Arabic),
    (2200, 2273, Script::Arabic),
    (2274, 2274, Script::Common),
    (2275, 2303, Script::Arabic),
    (2304, 2384, Script::Devanagari),
    (2385, 2388, Script::Inherited),
    (2389, 2403, Script::Devanagari),
    (2404, 2405, Script::Common),
    (2406, 2431, Script::Devanagari),
    (2432, 2435, Script::Bengali),
    (2437, 2444, Script::Bengali),
    (2447, 2448, Script::Bengali),
    (2451, 2472, Script::Bengali),
    (2474, 2480, Script::Bengali),
    (2482, 2482, Script::Bengali),
    (2486, 2489, Script::Bengali),
    (2492, 2500, Script::Bengali),
    (2503, 2504, Script::Bengali),
    (2507, 2510, Script::Bengali),
    (2519, 2519, Script::Bengali),
    (2524, 2525, Script::Bengali),
    (2527, 2531, Script::Bengali),
    (2534, 2558, Script::Bengali),
    (2561, 2563, Script::Gurmukhi),
    (2565, 2570, Script::Gurmukhi),
    (2575, 2576, Script::Gurmukhi),
    (2579, 2600, Script::Gurmukhi),
    (2602, 2608, Script::Gurmukhi),
    (2610, 2611, Script::Gurmukhi),
    (2613, 2614, Script::Gurmukhi),
    (2616, 2617, Script::Gurmukhi),
    (2620, 2620, Script::Gurmukhi),
    (2622, 2626, Script::Gurmukhi),
    (2631, 2632, Script::Gurmukhi),
    (2635, 2637, Script::Gurmukhi),
    (2641, 2641, Script::Gurmukhi),
    (2649, 2652, Script::Gurmukhi),
    (2654, 2654, Script::Gurmukhi),
    (2662, 2678, Script::Gurmukhi),
    (2689, 2691, Script::Gujarati),
    (2693, 2701, Script::Gujarati),
    (2703, 2705, Script::Gujarati),
    (2707, 2728, Script::Gujarati),
    (2730, 2736, Script::Gujarati),
    (2738, 2739, Script::Gujarati),
    (2741, 2745, Script::Gujarati),
    (2748, 2757, Script::Gujarati),
    (2759, 2761, Script::Gujarati),
    (2763, 2765, Script::Gujarati),
    (2768, 2768, Script::Gujarati),
    (2784, 2787, Script::Gujarati),
    (2790, 2801, Script::Gujarati),
    (2809, 2815, Script::Gujarati),
    (2817, 2819, Script::Oriya),
    (2821, 2828, Script::Oriya),
    (2831, 2832, Script::Oriya),
    (2835, 2856, Script::Oriya),
    (2858, 2864, Script::Oriya),
    (2866, 2867, Script::Oriya),
    (2869, 2873, Script::Oriya),
    (2876, 2884, Script::Oriya),
    (2887, 2888, Script::Oriya),
    (2891, 2893, Script::Oriya),
    (2901, 2903, Script::Oriya),
    (2908, 2909, Script::Oriya),
    (2911, 2915, Script::Oriya),
    (2918, 2935, Script::Oriya),
    (2946, 2947, Script::Tamil),
    (2949, 2954, Script::Tamil),
    (2958, 2960, Script::Tamil),
    (2962, 2965, Script::Tamil),
    (2969, 2970, Script::Tamil),
    (2972, 2972, Script::Tamil),
    (2974, 2975, Script::Tamil),
    (2979, 2980, Script::Tamil),
    (2984, 2986, Script::Tamil),
    (2990, 3001, Script::Tamil),
    (3006, 3010, Script::Tamil),
    (3014, 3016, Script::Tamil),
    (3018, 3021, Script::Tamil),
    (3024, 3024, Script::Tamil),
    (3031, 3031, Script::Tamil),
    (3046, 3066, Script::Tamil),
    (3072, 3084, Script::Telugu),
    (3086, 3088, Script::Telugu),
    (3090, 3112, Script::Telugu),
    (3114, 3129, Script::Telugu),
    (3132, 3140, Script::Telugu),
    (3142, 3144, Script::Telugu),
    (3146, 3149, Script::Telugu),
    (3157, 3158, Script::Telugu),
    (3160, 3162, Script::Telugu),
    (3165, 3165, Script::Telugu),
    (3168, 3171, Script::Telugu),
    (3174, 3183, Script::Telugu),
    (3191, 3199, Script::Telugu),
    (3200, 3212, Script::Kannada),
    (3214, 3216, Script::Kannada),
    (3218, 3240, Script::Kannada),
    (3242, 3251, Script::Kannada),
    (3253, 3257, Script::Kannada),
    (3260, 3268, Script::Kannada),
    (3270, 3272, Script::Kannada),
    (3274, 3277, Script::Kannada),
    (3285, 3286, Script::Kannada),
    (3293, 3294, Script::Kannada),
    (3296, 3299, Script::Kannada),
    (3302, 3311, Script::Kannada),
    (3313, 3315, Script::Kannada),
    (3328, 3340, Script::Malayalam),
    (3342, 3344, Script::Malayalam),
    (3346, 3396, Script::Malayalam),
    (3398, 3400, Script::Malayalam),
    (3402, 3407, Script::Malayalam),
    (3412, 3427, Script::Malayalam),
    (3430, 3455, Script::Malayalam),
    (3457, 3459, Script::Sinhala),
    (3461, 3478, Script::Sinhala),
    (3482, 3505, Script::Sinhala),
    (3507, 3515, Script::Sinhala),
    (3517, 3517, Script::Sinhala),
    (3520, 3526, Script::Sinhala),
    (3530, 3530, Script::Sinhala),
    (3535, 3540, Script::Sinhala),
    (3542, 3542, Script::Sinhala),
    (3544, 3551, Script::Sinhala),
    (3558, 3567, Script::Sinhala),
    (3570, 3572, Script::Sinhala),
    (3585, 3642, Script::Thai),
    (3647, 3647, Script::Common),
    (3648, 3675, Script::Thai),
    (3713, 3714, Script::Lao),
    (3716, 3716, Script::Lao),
    (3718, 3722, Script::Lao),
    (3724, 3747, Script::Lao),
    (3749, 3749, Script::Lao),
    (3751, 3773, Script::Lao),
    (3776, 3780, Script::Lao),
    (3782, 3782, Script::Lao),
    (3784, 3790, Script::Lao),
    (3792, 3801, Script::Lao),
    (3804, 3807, Script::Lao),
    (3840, 3911, Script::Tibetan),
    (3913, 3948, Script::Tibetan),
    (3953, 3991, Script::Tibetan),
    (3993, 4028, Script::Tibetan),
    (4030, 4044, Script::Tibetan),
    (4046, 4052, Script::Tibetan),
    (4053, 4056, Script::Common),
    (4057, 4058, Script::Tibetan),
    (4096, 4255, Script::Myanmar),
    (4256, 4293, Script::Georgian),
    (4295, 4295, Script::Georgian),
    (4301, 4301, Script::Georgian),
    (4304, 4346, Script::Georgian),
    (4347, 4347, Script::Common),
    (4348, 4351, Script::Georgian),
    (4352, 4607, Script::Hangul),
    (4608, 4680, Script::Ethiopic),
    (4682, 4685, Script::Ethiopic),
    (4688, 4694, Script::Ethiopic),
    (4696, 4696, Script::Ethiopic),
    (4698, 4701, Script::Ethiopic),
    (4704, 4744, Script::Ethiopic),
    (4746, 4749, Script::Ethiopic),
    (4752, 4784, Script::Ethiopic),
    (4786, 4789, Script::Ethiopic),
    (4792, 4798, Script::Ethiopic),
    (4800, 4800, Script::Ethiopic),
    (4802, 4805, Script::Ethiopic),
    (4808, 4822, Script::Ethiopic),
    (4824, 4880, Script::Ethiopic),
    (4882, 4885, Script::Ethiopic),
    (4888, 4954, Script::Ethiopic),
    (4957, 4988, Script::Ethiopic),
    (4992, 5017, Script::Ethiopic),
    (5024, 5109, Script::Cherokee),
    (5112, 5117, Script::Cherokee),
    (5120, 5759, Script::CanadianAboriginal),
    (5760, 5788, Script::Ogham),
    (5792, 5866, Script::Runic),
    (5867, 5869, Script::Common),
    (5870, 5880, Script::Runic),
    (5888, 5909, Script::Tagalog),
    (5919, 5919, Script::Tagalog),
    (5920, 5940, Script::Hanunoo),
    (5941, 5942, Script::Common),
    (5952, 5971, Script::Buhid),
    (5984, 5996, Script::Tagbanwa),
    (5998, 6000, Script::Tagbanwa),
    (6002, 6003, Script::Tagbanwa),
    (6016, 6109, Script::Khmer),
    (6112, 6121, Script::Khmer),
    (6128, 6137, Script::Khmer),
    (6144, 6145, Script::Mongolian),
    (6146, 6147, Script::Common),
    (6148, 6148, Script::Mongolian),
    (6149, 6149, Script::Common),
    (6150, 6169, Script::Mongolian),
    (6176, 6264, Script::Mongolian),
    (6272, 6314, Script::Mongolian),
    (6320, 6389, Script::CanadianAboriginal),
    (6400, 6430, Script::Limbu),
    (6432, 6443, Script::Limbu),
    (6448, 6459, Script::Limbu),
    (6464, 6464, Script::Limbu),
    (6468, 6479, Script::Limbu),
    (6480, 6509, Script::TaiLe),
    (6512, 6516, Script::TaiLe),
    (6528, 6571, Script::NewTaiLue),
    (6576, 6601, Script::NewTaiLue),
    (6608, 6618, Script::NewTaiLue),
    (6622, 6623, Script::NewTaiLue),
    (6624, 6655, Script::Khmer),
    (6656, 6683, Script::Buginese),
    (6686, 6687, Script::Buginese),
    (6688, 6750, Script::TaiTham),
    (6752, 6780, Script::TaiTham),
    (6783, 6793, Script::TaiTham),
    (6800, 6809, Script::TaiTham),
    (6816, 6829, Script::TaiTham),
    (6832, 6862, Script::Inherited),
    (6912, 6988, Script::Balinese),
    (6992, 7038, Script::Balinese),
    (7040, 7103, Script::Sundanese),
    (7104, 7155, Script::Batak),
    (7164, 7167, Script::Batak),
    (7168, 7223, Script::Lepcha),
    (7227, 7241, Script::Lepcha),
    (7245, 7247, Script::Lepcha),
    (7248, 7295, Script::OlChiki),
    (7296, 7304, Script::Cyrillic),
    (7312, 7354, Script::Georgian),
    (7357, 7359, Script::Georgian),
    (7360, 7367, Script::Sundanese),
    (7376, 7378, Script::Inherited),
    (7379, 7379, Script::Common),
    (7380, 7392, Script::Inherited),
    (7393, 7393, Script::Common),
    (7394, 7400, Script::Inherited),
    (7401, 7404, Script::Common),
    (7405, 7405, Script::Inherited),
    (7406, 7411, Script::Common),
    (7412, 7412, Script::Inherited),
    (7413, 7415, Script::Common),
    (7416, 7417, Script::Inherited),
    (7418, 7418, Script::Common),
    (7424, 7461, Script::Latin),
    (7462, 7466, Script::Greek),
    (7467, 7467, Script::Cyrillic),
    (7468, 7516, Script::Latin),
    (7517, 7521, Script::Greek),
    (7522, 7525, Script::Latin),
    (7526, 7530, Script::Greek),
    (7531, 7543, Script::Latin),
    (7544, 7544, Script::Cyrillic),
    (7545, 7614, Script::Latin),
    (7615, 7615, Script::Greek),
    (7616, 7679, Script::Inherited),
    (7680, 7935, Script::Latin),
    (7936, 7957, Script::Greek),
    (7960, 7965, Script::Greek),
    (7968, 8005, Script::Greek),
    (8008, 8013, Script::Greek),
    (8016, 8023, Script::Greek),
    (8025, 8025, Script::Greek),
    (8027, 8027, Script::Greek),
    (8029, 8029, Script::Greek),
    (8031, 8061, Script::Greek),
    (8064, 8116, Script::Greek),
    (8118, 8132, Script::Greek),
    (8134, 8147, Script::Greek),
    (8150, 8155, Script::Greek),
    (8157, 8175, Script::Greek),
    (8178, 8180, Script::Greek),
    (8182, 8190, Script::Greek),
    (8192, 8203, Script::Common),
    (8204, 8205, Script::Inherited),
    (8206, 8292, Script::Common),
    (8294, 8304, Script::Common),
    (8305, 8305, Script::Latin),
    (8308, 8318, Script::Common),
    (8319, 8319, Script::Latin),
    (8320, 8334, Script::Common),
    (8336, 8348, Script::Latin),
    (8352, 8384, Script::Common),
    (8400, 8432, Script::Inherited),
    (8448, 8485, Script::Common),
    (8486, 8486, Script::Greek),
    (8487, 8489, Script::Common),
    (8490, 8491, Script::Latin),
    (8492, 8497, Script::Common),
    (8498, 8498, Script::Latin),
    (8499, 8525, Script::Common),
    (8526, 8526, Script::Latin),
    (8527, 8543, Script::Common),
    (8544, 8584, Script::Latin),
    (8585, 8587, Script::Common),
    (8592, 9254, Script::Common),
    (9280, 9290, Script::Common),
    (9312, 10239, Script::Common),
    (10240, 10495, Script::Braille),
    (10496, 11123, Script::Common),
    (11126, 11157, Script::Common),
    (11159, 11263, Script::Common),
    (11264, 11359, Script::Glagolitic),
    (11360, 11391, Script::Latin),
    (11392, 11507, Script::Coptic),
    (11513, 11519, Script::Coptic),
    (11520, 11557, Script::Georgian),
    (11559, 11559, Script::Georgian),
    (11565, 11565, Script::Georgian),
    (11568, 11623, Script::Tifinagh),
    (11631, 11632, Script::Tifinagh),
    (11647, 11647, Script::Tifinagh),
    (11648, 11670, Script::Ethiopic),
    (11680, 11686, Script::Ethiopic),
    (11688, 11694, Script::Ethiopic),
    (11696, 11702, Script::Ethiopic),
    (11704, 11710, Script::Ethiopic),
    (11712, 11718, Script::Ethiopic),
    (11720, 11726, Script::Ethiopic),
    (11728, 11734, Script::Ethiopic),
    (11736, 11742, Script::Ethiopic),
    (11744, 11775, Script::Cyrillic),
    (11776, 11869, Script::Common),
    (11904, 11929, Script::Han),
    (11931, 12019, Script::Han),
    (12032, 12245, Script::Han),
    (12272, 12283, Script::Common),
    (12288, 12292, Script::Common),
    (12293, 12293, Script::Han),
    (12294, 12294, Script::Common),
    (12295, 12295, Script::Han),
    (12296, 12320, Script::Common),
    (12321, 12329, Script::Han),
    (12330, 12333, Script::Inherited),
    (12334, 12335, Script::Hangul),
    (12336, 12343, Script::Common),
    (12344, 12347, Script::Han),
    (12348, 12351, Script::Common),
    (12353, 12438, Script::Hiragana),
    (12441, 12442, Script::Inherited),
    (12443, 12444, Script::Common),
    (12445, 12447, Script::Hiragana),
    (12448, 12448, Script::Common),
    (12449, 12538, Script::Katakana),
    (12539, 12540, Script::Common),
    (12541, 12543, Script::Katakana),
    (12549, 12591, Script::Bopomofo),
    (12593, 12686, Script::Hangul),
    (12688, 12703, Script::Common),
    (12704, 12735, Script::Bopomofo),
    (12736, 12771, Script::Common),
    (12784, 12799, Script::Katakana),
    (12800, 12830, Script::Hangul),
    (12832, 12895, Script::Common),
    (12896, 12926, Script::Hangul),
    (12927, 13007, Script::Common),
    (13008, 13054, Script::Katakana),
    (13055, 13055, Script::Common),
    (13056, 13143, Script::Katakana),
    (13144, 13311, Script::Common),
    (13312, 19903, Script::Han),
    (19904, 19967, Script::Common),
    (19968, 40959, Script::Han),
    (40960, 42124, Script::Yi),
    (42128, 42182, Script::Yi),
    (42192, 42239, Script::Lisu),
    (42240, 42539, Script::Vai),
    (42560, 42655, Script::Cyrillic),
    (42656, 42743, Script::Bamum),
    (42752, 42785, Script::Common),
    (42786, 42887, Script::Latin),
    (42888, 42890, Script::Common),
    (42891, 42954, Script::Latin),
    (42960, 42961, Script::Latin),
    (42963, 42963, Script::Latin),
    (42965, 42969, Script::Latin),
    (42994, 43007, Script::Latin),
    (43008, 43052, Script::SylotiNagri),
    (43056, 43065, Script::Common),
    (43072, 43127, Script::PhagsPa),
    (43136, 43205, Script::Saurashtra),
    (43214, 43225, Script::Saurashtra),
    (43232, 43263, Script::Devanagari),
    (43264, 43309, Script::KayahLi),
    (43310, 43310, Script::Common),
    (43311, 43311, Script::KayahLi),
    (43312, 43347, Script::Rejang),
    (43359, 43359, Script::Rejang),
    (43360, 43388, Script::Hangul),
    (43392, 43469, Script::Javanese),
    (43471, 43471, Script::Common),
    (43472, 43481, Script::Javanese),
    (43486, 43487, Script::Javanese),
    (43488, 43518, Script::Myanmar),
    (43520, 43574, Script::Cham),
    (43584, 43597, Script::Cham),
    (43600, 43609, Script::Cham),
    (43612, 43615, Script::Cham),
    (43616, 43647, Script::Myanmar),
    (43648, 43714, Script::TaiViet),
    (43739, 43743, Script::TaiViet),
    (43744, 43766, Script::MeeteiMayek),
    (43777, 43782, Script::Ethiopic),
    (43785, 43790, Script::Ethiopic),
    (43793, 43798, Script::Ethiopic),
    (43808, 43814, Script::Ethiopic),
    (43816, 43822, Script::Ethiopic),
    (43824, 43866, Script::Latin),
    (43867, 43867, Script::Common),
    (43868, 43876, Script::Latin),
    (43877, 43877, Script::Greek),
    (43878, 43881, Script::Latin),
    (43882, 43883, Script::Common),
    (43888, 43967, Script::Cherokee),
    (43968, 44013, Script::MeeteiMayek),
    (44016, 44025, Script::MeeteiMayek),
    (44032, 55203, Script::Hangul),
    (55216, 55238, Script::Hangul),
    (55243, 55291, Script::Hangul),
    (63744, 64109, Script::Han),
    (64112, 64217, Script::Han),
    (64256, 64262, Script::Latin),
    (64275, 64279, Script::Armenian),
    (64285, 64310, Script::Hebrew),
    (64312, 64316, Script::Hebrew),
    (64318, 64318, Script::Hebrew),
    (64320, 64321, Script::Hebrew),
    (64323, 64324, Script::Hebrew),
    (64326, 64335, Script::Hebrew),
    (64336, 64450, Script::Arabic),
    (64467, 64829, Script::Arabic),
    (64830, 64831, Script::Common),
    (64832, 64911, Script::Arabic),
    (64914, 64967, Script::Arabic),
    (64975, 64975, Script::Arabic),
    (65008, 65023, Script::Arabic),
    (65024, 65039, Script::Inherited),
    (65040, 65049, Script::Common),
    (65056, 65069, Script::Inherited),
    (65070, 65071, Script::Cyrillic),
    (65072, 65106, Script::Common),
    (65108, 65126, Script::Common),
    (65128, 65131, Script::Common),
    (65136, 65140, Script::Arabic),
    (65142, 65276, Script::Arabic),
    (65279, 65279, Script::Common),
    (65281, 65312, Script::Common),
    (65313, 65338, Script::Latin),
    (65339, 65344, Script::Common),
    (65345, 65370, Script::Latin),
    (65371, 65381, Script::Common),
    (65382, 65391, Script::Katakana),
    (65392, 65392, Script::Common),
    (65393, 65437, Script::Katakana),
    (65438, 65439, Script::Common),
    (65440, 65470, Script::Hangul),
    (65474, 65479, Script::Hangul),
    (65482, 65487, Script::Hangul),
    (65490, 65495, Script::Hangul),
    (65498, 65500, Script::Hangul),
    (65504, 65510, Script::Common),
    (65512, 65518, Script::Common),
    (65529, 65533, Script::Common),
    (65536, 65547, Script::LinearB),
    (65549, 65574, Script::LinearB),
    (65576, 65594, Script::LinearB),
    (65596, 65597, Script::LinearB),
    (65599, 65613, Script::LinearB),
    (65616, 65629, Script::LinearB),
    (65664, 65786, Script::LinearB),
    (65792, 65794, Script::Common),
    (65799, 65843, Script::Common),
    (65847, 65855, Script::Common),
    (65856, 65934, Script::Greek),
    (65936, 65948, Script::Common),
    (65952, 65952, Script::Greek),
    (66000, 66044, Script::Common),
    (66045, 66045, Script::Inherited),
    (66176, 66204, Script::Lycian),
    (66208, 66256, Script::Carian),
    (66272, 66272, Script::Inherited),
    (66273, 66299, Script::Common),
    (66304, 66339, Script::OldItalic),
    (66349, 66351, Script::OldItalic),
    (66352, 66378, Script::Gothic),
    (66384, 66426, Script::OldPermic),
    (66432, 66461, Script::Ugaritic),
    (66463, 66463, Script::Ugaritic),
    (66464, 66499, Script::OldPersian),
    (66504, 66517, Script::OldPersian),
    (66560, 66639, Script::Deseret),
    (66640, 66687, Script::Shavian),
    (66688, 66717, Script::Osmanya),
    (66720, 66729, Script::Osmanya),
    (66736, 66771, Script::Osage),
    (66776, 66811, Script::Osage),
    (66816, 66855, Script::Elbasan),
    (66864, 66915, Script::CaucasianAlbanian),
    (66927, 66927, Script::CaucasianAlbanian),
    (66928, 66938, Script::Vithkuqi),
    (66940, 66954, Script::Vithkuqi),
    (66956, 66962, Script::Vithkuqi),
    (66964, 66965, Script::Vithkuqi),
    (66967, 66977, Script::Vithkuqi),
    (66979, 66993, Script::Vithkuqi),
    (66995, 67001, Script::Vithkuqi),
    (67003, 67004, Script::Vithkuqi),
    (67072, 67382, Script::LinearA),
    (67392, 67413, Script::LinearA),
    (67424, 67431, Script::LinearA),
    (67456, 67461, Script::Latin),
    (67463, 67504, Script::Latin),
    (67506, 67514, Script::Latin),
    (67584, 67589, Script::Cypriot),
    (67592, 67592, Script::Cypriot),
    (67594, 67637, Script::Cypriot),
    (67639, 67640, Script::Cypriot),
    (67644, 67644, Script::Cypriot),
    (67647, 67647, Script::Cypriot),
    (67648, 67669, Script::ImperialAramaic),
    (67671, 67679, Script::ImperialAramaic),
    (67680, 67711, Script::Palmyrene),
    (67712, 67742, Script::Nabataean),
    (67751, 67759, Script::Nabataean),
    (67808, 67826, Script::Hatran),
    (67828, 67829, Script::Hatran),
    (67835, 67839, Script::Hatran),
    (67840, 67867, Script::Phoenician),
    (67871, 67871, Script::Phoenician),
    (67872, 67897, Script::Lydian),
    (67903, 67903, Script::Lydian),
    (67968, 67999, Script::MeroiticHieroglyphs),
    (68000, 68023, Script::MeroiticCursive),
    (68028, 68047, Script::MeroiticCursive),
    (68050, 68095, Script::MeroiticCursive),
    (68096, 68099, Script::Kharoshthi),
    (68101, 68102, Script::Kharoshthi),
    (68108, 68115, Script::Kharoshthi),
    (68117, 68119, Script::Kharoshthi),
    (68121, 68149, Script::Kharoshthi),
    (68152, 68154, Script::Kharoshthi),
    (68159, 68168, Script::Kharoshthi),
    (68176, 68184, Script::Kharoshthi),
    (68192, 68223, Script::OldSouthArabian),
    (68224, 68255, Script::OldNorthArabian),
    (68288, 68326, Script::Manichaean),
    (68331, 68342, Script::Manichaean),
    (68352, 68405, Script::Avestan),
    (68409, 68415, Script::Avestan),
    (68416, 68437, Script::InscriptionalParthian),
    (68440, 68447, Script::InscriptionalParthian),
    (68448, 68466, Script::InscriptionalPahlavi),
    (68472, 68479, Script::InscriptionalPahlavi),
    (68480, 68497, Script::PsalterPahlavi),
    (68505, 68508, Script::PsalterPahlavi),
    (68521, 68527, Script::PsalterPahlavi),
    (68608, 68680, Script::OldTurkic),
    (68736, 68786, Script::OldHungarian),
    (68800, 68850, Script::OldHungarian),
    (68858, 68863, Script::OldHungarian),
    (68864, 68903, Script::HanifiRohingya),
    (68912, 68921, Script::HanifiRohingya),
    (69216, 69246, Script::Arabic),
    (69248, 69289, Script::Yezidi),
    (69291, 69293, Script::Yezidi),
    (69296, 69297, Script::Yezidi),
    (69373, 69375, Script::Arabic),
    (69376, 69415, Script::OldSogdian),
    (69424, 69465, Script::Sogdian),
    (69488, 69513, Script::OldUyghur),
    (69552, 69579, Script::Chorasmian),
    (69600, 69622, Script::Elymaic),
    (69632, 69709, Script::Brahmi),
    (69714, 69749, Script::Brahmi),
    (69759, 69759, Script::Brahmi),
    (69760, 69826, Script::Kaithi),
    (69837, 69837, Script::Kaithi),
    (69840, 69864, Script::SoraSompeng),
    (69872, 69881, Script::SoraSompeng),
    (69888, 69940, Script::Chakma),
    (69942, 69959, Script::Chakma),
    (69968, 70006, Script::Mahajani),
    (70016, 70111, Script::Sharada),
    (70113, 70132, Script::Sinhala),
    (70144, 70161, Script::Khojki),
    (70163, 70209, Script::Khojki),
    (70272, 70278, Script::Multani),
    (70280, 70280, Script::Multani),
    (70282, 70285, Script::Multani),
    (70287, 70301, Script::Multani),
    (70303, 70313, Script::Multani),
    (70320, 70378, Script::Khudawadi),
    (70384, 70393, Script::Khudawadi),
    (70400, 70403, Script::Grantha),
    (70405, 70412, Script::Grantha),
    (70415, 70416, Script::Grantha),
    (70419, 70440, Script::Grantha),
    (70442, 70448, Script::Grantha),
    (70450, 70451, Script::Grantha),
    (70453, 70457, Script::Grantha),
    (70459, 70459, Script::Inherited),
    (70460, 70468, Script::Grantha),
    (70471, 70472, Script::Grantha),
    (70475, 70477, Script::Grantha),
    (70480, 70480, Script::Grantha),
    (70487, 70487, Script::Grantha),
    (70493, 70499, Script::Grantha),
    (70502, 70508, Script::Grantha),
    (70512, 70516, Script::Grantha),
    (70656, 70747, Script::Newa),
    (70749, 70753, Script::Newa),
    (70784, 70855, Script::Tirhuta),
    (70864, 70873, Script::Tirhuta),
    (71040, 71093, Script::Siddham),
    (71096, 71133, Script::Siddham),
    (71168, 71236, Script::Modi),
    (71248, 71257, Script::Modi),
    (71264, 71276, Script::Mongolian),
    (71296, 71353, Script::Takri),
    (71360, 71369, Script::Takri),
    (71424, 71450, Script::Ahom),
    (71453, 71467, Script::Ahom),
    (71472, 71494, Script::Ahom),
    (71680, 71739, Script::Dogra),
    (71840, 71922, Script::WarangCiti),
    (71935, 71935, Script::WarangCiti),
    (71936, 71942, Script::DivesAkuru),
    (71945, 71945, Script::DivesAkuru),
    (71948, 71955, Script::DivesAkuru),
    (71957, 71958, Script::DivesAkuru),
    (71960, 71989, Script::DivesAkuru),
    (71991, 71992, Script::DivesAkuru),
    (71995, 72006, Script::DivesAkuru),
    (72016, 72025, Script::DivesAkuru),
    (72096, 72103, Script::Nandinagari),
    (72106, 72151, Script::Nandinagari),
    (72154, 72164, Script::Nandinagari),
    (72192, 72263, Script::ZanabazarSquare),
    (72272, 72354, Script::Soyombo),
    (72368, 72383, Script::CanadianAboriginal),
    (72384, 72440, Script::PauCinHau),
    (72448, 72457, Script::Devanagari),
    (72704, 72712, Script::Bhaiksuki),
    (72714, 72758, Script::Bhaiksuki),
    (72760, 72773, Script::Bhaiksuki),
    (72784, 72812, Script::Bhaiksuki),
    (72816, 72847, Script::Marchen),
    (72850, 72871, Script::Marchen),
    (72873, 72886, Script::Marchen),
    (72960, 72966, Script::MasaramGondi),
    (72968, 72969, Script::MasaramGondi),
    (72971, 73014, Script::MasaramGondi),
    (73018, 73018, Script::MasaramGondi),
    (73020, 73021, Script::MasaramGondi),
    (73023, 73031, Script::MasaramGondi),
    (73040, 73049, Script::MasaramGondi),
    (73056, 73061, Script::GunjalaGondi),
    (73063, 73064, Script::GunjalaGondi),
    (73066, 73102, Script::GunjalaGondi),
    (73104, 73105, Script::GunjalaGondi),
    (73107, 73112, Script::GunjalaGondi),
    (73120, 73129, Script::GunjalaGondi),
    (73440, 73464, Script::Makasar),
    (73472, 73488, Script::Kawi),
    (73490, 73530, Script::Kawi),
    (73534, 73561, Script::Kawi),
    (73648, 73648, Script::Lisu),
    (73664, 73713, Script::Tamil),
    (73727, 73727, Script::Tamil),
    (73728, 74649, Script::Cuneiform),
    (74752, 74862, Script::Cuneiform),
    (74864, 74868, Script::Cuneiform),
    (74880, 75075, Script::Cuneiform),
    (77712, 77810, Script::CyproMinoan),
    (77824, 78933, Script::EgyptianHieroglyphs),
    (82944, 83526, Script::AnatolianHieroglyphs),
    (92160, 92728, Script::Bamum),
    (92736, 92766, Script::Mro),
    (92768, 92777, Script::Mro),
    (92782, 92783, Script::Mro),
    (92784, 92862, Script::Tangsa),
    (92864, 92873, Script::Tangsa),
    (92880, 92909, Script::BassaVah),
    (92912, 92917, Script::BassaVah),
    (92928, 92997, Script::PahawhHmong),
    (93008, 93017, Script::PahawhHmong),
    (93019, 93025, Script::PahawhHmong),
    (93027, 93047, Script::PahawhHmong),
    (93053, 93071, Script::PahawhHmong),
    (93760, 93850, Script::Medefaidrin),
    (93952, 94026, Script::Miao),
    (94031, 94087, Script::Miao),
    (94095, 94111, Script::Miao),
    (94176, 94176, Script::Tangut),
    (94177, 94177, Script::Nushu),
    (94178, 94179, Script::Han),
    (94180, 94180, Script::KhitanSmallScript),
    (94192, 94193, Script::Han),
    (94208, 100343, Script::Tangut),
    (100352, 101119, Script::Tangut),
    (101120, 101589, Script::KhitanSmallScript),
    (101632, 101640, Script::Tangut),
    (110576, 110579, Script::Katakana),
    (110581, 110587, Script::Katakana),
    (110589, 110590, Script::Katakana),
    (110592, 110592, Script::Katakana),
    (110593, 110879, Script::Hiragana),
    (110880, 110882, Script::Katakana),
    (110898, 110898, Script::Hiragana),
    (110928, 110930, Script::Hiragana),
    (110933, 110933, Script::Katakana),
    (110948, 110951, Script::Katakana),
    (110960, 111355, Script::Nushu),
    (113664, 113770, Script::Duployan),
    (113776, 113788, Script::Duployan),
    (113792, 113800, Script::Duployan),
    (113808, 113817, Script::Duployan),
    (113820, 113823, Script::Duployan),
    (113824, 113827, Script::Common),
    (118528, 118573, Script::Inherited),
    (118576, 118598, Script::Inherited),
    (118608, 118723, Script::Common),
    (118784, 119029, Script::Common),
    (119040, 119078, Script::Common),
    (119081, 119142, Script::Common),
    (119143, 119145, Script::Inherited),
    (119146, 119162, Script::Common),
    (119163, 119170, Script::Inherited),
    (119171, 119172, Script::Common),
    (119173, 119179, Script::Inherited),
    (119180, 119209, Script::Common),
    (119210, 119213, Script::Inherited),
    (119214, 119274, Script::Common),
    (119296, 119365, Script::Greek),
    (119488, 119507, Script::Common),
    (119520, 119539, Script::Common),
    (119552, 119638, Script::Common),
    (119648, 119672, Script::Common),
    (119808, 119892, Script::Common),
    (119894, 119964, Script::Common),
    (119966, 119967, Script::Common),
    (119970, 119970, Script::Common),
    (119973, 119974, Script::Common),
    (119977, 119980, Script::Common),
    (119982, 119993, Script::Common),
    (119995, 119995, Script::Common),
    (119997, 120003, Script::Common),
    (120005, 120069, Script::Common),
    (120071, 120074, Script::Common),
    (120077, 120084, Script::Common),
    (120086, 120092, Script::Common),
    (120094, 120121, Script::Common),
    (120123, 120126, Script::Common),
    (120128, 120132, Script::Common),
    (120134, 120134, Script::Common),
    (120138, 120144, Script::Common),
    (120146, 120485, Script::Common),
    (120488, 120779, Script::Common),
    (120782, 120831, Script::Common),
    (120832, 121483, Script::Signwriting),
    (121499, 121503, Script::Signwriting),
    (121505, 121519, Script::Signwriting),
    (122624, 122654, Script::Latin),
    (122661, 122666, Script::Latin),
    (122880, 122886, Script::Glagolitic),
    (122888, 122904, Script::Glagolitic),
    (122907, 122913, Script::Glagolitic),
    (122915, 122916, Script::Glagolitic),
    (122918, 122922, Script::Glagolitic),
    (122928, 122989, Script::Cyrillic),
    (123023, 123023, Script::Cyrillic),
    (123136, 123180, Script::NyiakengPuachueHmong),
    (123184, 123197, Script::NyiakengPuachueHmong),
    (123200, 123209, Script::NyiakengPuachueHmong),
    (123214, 123215, Script::NyiakengPuachueHmong),
    (123536, 123566, Script::Toto),
    (123584, 123641, Script::Wancho),
    (123647, 123647, Script::Wancho),
    (124112, 124153, Script::NagMundari),
    (124896, 124902, Script::Ethiopic),
    (124904, 124907, Script::Ethiopic),
    (124909, 124910, Script::Ethiopic),
    (124912, 124926, Script::Ethiopic),
    (124928, 125124, Script::MendeKikakui),
    (125127, 125142, Script::MendeKikakui),
    (125184, 125259, Script::Adlam),
    (125264, 125273, Script::Adlam),
    (125278, 125279, Script::Adlam),
    (126065, 126132, Script::Common),
    (126209, 126269, Script::Common),
    (126464, 126467, Script::Arabic),
    (126469, 126495, Script::Arabic),
    (126497, 126498, Script::Arabic),
    (126500, 126500, Script::Arabic),
    (126503, 126503, Script::Arabic),
    (126505, 126514, Script::Arabic),
    (126516, 126519, Script::Arabic),
    (126521, 126521, Script::Arabic),
    (126523, 126523, Script::Arabic),
    (126530, 126530, Script::Arabic),
    (126535, 126535, Script::Arabic),
    (126537, 126537, Script::Arabic),
    (126539, 126539, Script::Arabic),
    (126541, 126543, Script::Arabic),
    (126545, 126546, Script::Arabic),
    (126548, 126548, Script::Arabic),
    (126551, 126551, Script::Arabic),
    (126553, 126553, Script::Arabic),
    (126555, 126555, Script::Arabic),
    (126557, 126557, Script::Arabic),
    (126559, 126559, Script::Arabic),
    (126561, 126562, Script::Arabic),
    (126564, 126564, Script::Arabic),
    (126567, 126570, Script::Arabic),
    (126572, 126578, Script::Arabic),
    (126580, 126583, Script::Arabic),
    (126585, 126588, Script::Arabic),
    (126590, 126590, Script::Arabic),
    (126592, 126601, Script::Arabic),
    (126603, 126619, Script::Arabic),
    (126625, 126627, Script::Arabic),
    (126629, 126633, Script::Arabic),
    (126635, 126651, Script::Arabic),
    (126704, 126705, Script::Arabic),
    (126976, 127019, Script::Common),
    (127024, 127123, Script::Common),
    (127136, 127150, Script::Common),
    (127153, 127167, Script::Common),
    (127169, 127183, Script::Common),
    (127185, 127221, Script::Common),
    (127232, 127405, Script::Common),
    (127462, 127487, Script::Common),
    (127488, 127488, Script::Hiragana),
    (127489, 127490, Script::Common),
    (127504, 127547, Script::Common),
    (127552, 127560, Script::Common),
    (127568, 127569, Script::Common),
    (127584, 127589, Script::Common),
    (127744, 128727, Script::Common),
    (128732, 128748, Script::Common),
    (128752, 128764, Script::Common),
    (128768, 128886, Script::Common),
    (128891, 128985, Script::Common),
    (128992, 129003, Script::Common),
    (129008, 129008, Script::Common),
    (129024, 129035, Script::Common),
    (129040, 129095, Script::Common),
    (129104, 129113, Script::Common),
    (129120, 129159, Script::Common),
    (129168, 129197, Script::Common),
    (129200, 129201, Script::Common),
    (129280, 129619, Script::Common),
    (129632, 129645, Script::Common),
    (129648, 129660, Script::Common),
    (129664, 129672, Script::Common),
    (129680, 129725, Script::Common),
    (129727, 129733, Script::Common),
    (129742, 129755, Script::Common),
    (129760, 129768, Script::Common),
    (129776, 129784, Script::Common),
    (129792, 129938, Script::Common),
    (129940, 129994, Script::Common),
    (130032, 130041, Script::Common),
    (131072, 173791, Script::Han),
    (173824, 177977, Script::Han),
    (177984, 178205, Script::Han),
    (178208, 183969, Script::Han),
    (183984, 191456, Script::Han),
    (194560, 195101, Script::Han),
    (196608, 201546, Script::Han),
    (201552, 205743, Script::Han),
    (917505, 917505, Script::Common),
    (917536, 917631, Script::Common),
    (917760, 917999, Script::Inherited),
];
