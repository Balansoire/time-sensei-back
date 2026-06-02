use super::FicheRevision;

pub fn generate() -> (Vec<FicheRevision>, Vec<FicheRevision>, Vec<FicheRevision>) {
    let kana = vec![
        // voyelles
        ("a","あ","ア","v","base"), ("i","い","イ","v","base"), ("u","う","ウ","v","base"),
        ("e","え","エ","v","base"), ("o","お","オ","v","base"),

        // K
        ("ka","か","カ","k","base"), ("ki","き","キ","k","base"), ("ku","く","ク","k","base"),
        ("ke","け","ケ","k","base"), ("ko","こ","コ","k","base"),

        // S
        ("sa","さ","サ","s","base"), ("shi","し","シ","s","base"), ("su","す","サ","s","base"),
        ("se","せ","セ","s","base"), ("so","そ","ソ","s","base"),

        // T
        ("ta","た","タ","t","base"), ("chi","ち","チ","t","base"), ("tsu","つ","ツ","t","base"),
        ("te","て","テ","t","base"), ("to","と","ト","t","base"),

        // N
        ("na","な","ナ","n","base"), ("ni","に","ニ","n","base"), ("nu","ぬ","ヌ","n","base"),
        ("ne","ね","ネ","n","base"), ("no","の","ノ","n","base"),

        // H
        ("ha","は","ハ","h","base"), ("hi","ひ","ヒ","h","base"), ("fu","ふ","フ","h","base"),
        ("he","へ","ヘ","h","base"), ("ho","ほ","ホ","h","base"),

        // M
        ("ma","ま","マ","m","base"), ("mi","み","ミ","m","base"), ("mu","む","ム","m","base"),
        ("me","め","メ","m","base"), ("mo","も","モ","m","base"),

        // Y
        ("ya","や","ヤ","y","base"), ("yu","ゆ","ユ","y","base"), ("yo","よ","ヨ","y","base"),

        // R
        ("ra","ら","ラ","r","base"), ("ri","り","リ","r","base"), ("ru","る","ル","r","base"),
        ("re","れ","レ","r","base"), ("ro","ろ","ロ","r","base"),

        // W
        ("wa","わ","ワ","w","base"), ("wo","を","ヲ","w","base"),

        // N
        ("n","ん","ン","n","special"),

        // G
        ("ga","が","ガ","g","dakuten"), ("gi","ぎ","ギ","g","dakuten"), ("gu","ぐ","グ","g","dakuten"),
        ("ge","げ","ゲ","g","dakuten"), ("go","ご","ゴ","g","dakuten"),

        // Z
        ("za","ざ","ザ","z","dakuten"), ("ji","じ","ジ","z","dakuten"), ("zu","ず","ズ","z","dakuten"),
        ("ze","ぜ","ゼ","z","dakuten"), ("zo","ぞ","ゾ","z","dakuten"),

        // D
        ("da","だ","ダ","d","dakuten"), ("ji","ぢ","ヂ","d","dakuten"), ("zu","づ","ヅ","d","dakuten"),
        ("de","で","デ","d","dakuten"), ("do","ど","ド","d","dakuten"),

        // B
        ("ba","ば","バ","b","dakuten"), ("bi","び","ビ","b","dakuten"), ("bu","ぶ","ブ","b","dakuten"),
        ("be","べ","ベ","b","dakuten"), ("bo","ぼ","ボ","b","dakuten"),

        // P
        ("pa","ぱ","パ","p","handakuten"), ("pi","ぴ","ピ","p","handakuten"), ("pu","ぷ","プ","p","handakuten"),
        ("pe","ぺ","ペ","p","handakuten"), ("po","ぽ","ポ","p","handakuten"),
        
        // Yōon (combinaisons avec や・ゆ・よ)
        ("kya","きゃ","キャ","ky","yoon"), ("kyu","きゅ","キュ","ky","yoon"), ("kyo","きょ","キョ","ky","yoon"),
        ("gya","ぎゃ","ギャ","gy","yoon"), ("gyu","ぎゅ","ギュ","gy","yoon"), ("gyo","ぎょ","ギョ","gy","yoon"),
        ("sha","しゃ","シャ","sh","yoon"), ("shu","しゅ","シュ","sh","yoon"), ("sho","しょ","ショ","sh","yoon"),
        ("ja","じゃ","ジャ","j","yoon"), ("ju","じゅ","ジュ","j","yoon"), ("jo","じょ","ジョ","j","yoon"),
        ("cha","ちゃ","チャ","ch","yoon"), ("chu","ちゅ","チュ","ch","yoon"), ("cho","ちょ","チョ","ch","yoon"),
        ("nya","にゃ","ニャ","ny","yoon"), ("nyu","にゅ","ニュ","ny","yoon"), ("nyo","にょ","ニョ","ny","yoon"),
        ("hya","ひゃ","ヒャ","hy","yoon"), ("hyu","ひゅ","ヒュ","hy","yoon"), ("hyo","ひょ","ヒョ","hy","yoon"),
        ("bya","びゃ","ビャ","by","yoon"), ("byu","びゅ","ビュ","by","yoon"), ("byo","びょ","ビョ","by","yoon"),
        ("pya","ぴゃ","ピャ","py","yoon"), ("pyu","ぴゅ","ピュ","py","yoon"), ("pyo","ぴょ","ピョ","py","yoon"),
        ("mya","みゃ","ミャ","my","yoon"), ("myu","みゅ","ミュ","my","yoon"), ("myo","みょ","ミョ","my","yoon"),
        ("rya","りゃ","リャ","ry","yoon"), ("ryu","りゅ","リュ","ry","yoon"), ("ryo","りょ","リョ","ry","yoon"),

        // kana historiques (optionnel mais parfois utile)
        ("wi","ゐ","ヰ","w","archaic"), ("we","ゑ","ヱ","w","archaic"),
    ];

    let mut hiragana = Vec::new();
    let mut katakana = Vec::new();

    for (r, h, k, g, sg) in kana {
        hiragana.push(FicheRevision::new_hiragana(r, h, sg, g));
        katakana.push(FicheRevision::new_katakana(r, k, sg, g));
    }
    
    let n5_kanji = vec![

        // =========================
        // NOMBRES
        // =========================
        ("ichi", "いち", "i", "い", "一", "nombres", "N5", "un"),
        ("ni", "に", "ni", "に", "二", "nombres", "N5", "deux"),
        ("san", "さん", "sa", "さ", "三", "nombres", "N5", "trois"),
        ("shi", "し", "shi", "し", "四", "nombres", "N5", "quatre"),
        ("go", "ご", "go", "ご", "五", "nombres", "N5", "cinq"),
        ("roku", "ろく", "ro", "ろ", "六", "nombres", "N5", "six"),
        ("shichi", "しち", "shi", "し", "七", "nombres", "N5", "sept"),
        ("hachi", "はち", "ha", "は", "八", "nombres", "N5", "huit"),
        ("kyu", "きゅう", "kyu", "き", "九", "nombres", "N5", "neuf"),
        ("juu", "じゅう", "ju", "じ", "十", "nombres", "N5", "dix"),

        // =========================
        // GRANDEURS
        // =========================
        ("hyaku", "ひゃく", "hya", "ひ", "百", "nombres", "N5", "cent"),
        ("sen", "せん", "se", "せ", "千", "nombres", "N5", "mille"),
        ("man", "まん", "ma", "ま", "万", "nombres", "N5", "dix mille"),
        ("en", "えん", "e", "え", "円", "nombres", "N5", "yen"),

        // =========================
        // TEMPS
        // =========================
        ("nichi", "にち", "ni", "に", "日", "temps", "N5", "jour / soleil"),
        ("getsu", "げつ", "ge", "げ", "月", "temps", "N5", "mois / lune"),
        ("ka", "か", "ka", "か", "火", "temps", "N5", "feu / mardi"),
        ("sui", "すい", "su", "す", "水", "temps", "N5", "eau / mercredi"),
        ("moku", "もく", "mo", "も", "木", "temps", "N5", "bois / jeudi"),
        ("kin", "きん", "ki", "き", "金", "temps", "N5", "or / vendredi"),
        ("do", "ど", "do", "ど", "土", "temps", "N5", "terre / samedi"),
        ("nen", "ねん", "ne", "ね", "年", "temps", "N5", "année"),
        ("ji", "じ", "ji", "じ", "時", "temps", "N5", "heure"),
        ("fun", "ふん", "fu", "ふ", "分", "temps", "N5", "minute"),
        ("shu", "しゅう", "shu", "し", "週", "temps", "N5", "semaine"),
        ("you", "よう", "yo", "よ", "曜", "temps", "N5", "jour de la semaine"),

        // =========================
        // POSITION / ESPACE
        // =========================
        ("ue", "うえ", "u", "う", "上", "position", "N5", "au-dessus"),
        ("shita", "した", "shi", "し", "下", "position", "N5", "en dessous"),
        ("hidari", "ひだり", "hi", "ひ", "左", "position", "N5", "gauche"),
        ("migi", "みぎ", "mi", "み", "右", "position", "N5", "droite"),
        ("naka", "なか", "na", "な", "中", "position", "N5", "centre / intérieur"),
        ("dai", "だい", "da", "だ", "大", "position", "N5", "grand"),
        ("shou", "しょう", "sho", "しょ", "小", "position", "N5", "petit"),

        // =========================
        // PERSONNES / CORPS
        // =========================
        ("hito", "ひと", "hi", "ひ", "人", "personnes", "N5", "personne"),
        ("onna", "おんな", "on", "お", "女", "personnes", "N5", "femme"),
        ("otoko", "おとこ", "o", "お", "男", "personnes", "N5", "homme"),
        ("ko", "こ", "ko", "こ", "子", "personnes", "N5", "enfant"),
        ("me", "め", "me", "め", "目", "corps", "N5", "œil"),
        ("mimi", "みみ", "mi", "み", "耳", "corps", "N5", "oreille"),
        ("kuchi", "くち", "ku", "く", "口", "corps", "N5", "bouche"),
        ("te", "て", "te", "て", "手", "corps", "N5", "main"),
        ("ashi", "あし", "a", "あ", "足", "corps", "N5", "pied"),
        ("chikara", "ちから", "chi", "ち", "力", "corps", "N5", "force"),
        ("ki", "き", "ki", "き", "気", "corps", "N5", "énergie / esprit"),

        // =========================
        // ACTIONS
        // =========================
        ("miru", "みる", "mi", "み", "見", "actions", "N5", "voir"),
        ("iku", "いく", "i", "い", "行", "actions", "N5", "aller"),
        ("kuru", "くる", "ku", "く", "来", "actions", "N5", "venir"),
        ("taberu", "たべる", "ta", "た", "食", "actions", "N5", "manger"),
        ("nomu", "のむ", "no", "の", "飲", "actions", "N5", "boire"),
        ("kaku", "かく", "ka", "か", "書", "actions", "N5", "écrire"),
        ("yomu", "よむ", "yo", "よ", "読", "actions", "N5", "lire"),
        ("hanasu", "はなす", "ha", "は", "話", "actions", "N5", "parler"),
        ("kiku", "きく", "ki", "き", "聞", "actions", "N5", "écouter"),
        ("yasumu", "やすむ", "ya", "や", "休", "actions", "N5", "se reposer"),

        // =========================
        // ÉCOLE / SOCIÉTÉ
        // =========================
        ("gaku", "がく", "ga", "が", "学", "école", "N5", "étude"),
        ("kou", "こう", "ko", "こ", "校", "école", "N5", "école"),
        ("sei", "せい", "se", "せ", "生", "école", "N5", "élève / vie"),
        ("sen", "せん", "se", "せ", "先", "école", "N5", "avant / professeur"),
        ("mei", "めい", "me", "め", "名", "école", "N5", "nom"),
        ("mae", "まえ", "ma", "ま", "前", "école", "N5", "devant"),
        ("sha", "しゃ", "sha", "し", "社", "école", "N5", "entreprise"),
        ("hon", "ほん", "ho", "ほ", "本", "objets", "N5", "livre"),

        // =========================
        // NATURE
        // =========================
        ("yama", "やま", "ya", "や", "山", "nature", "N5", "montagne"),
        ("kawa", "かわ", "ka", "か", "川", "nature", "N5", "rivière"),
        ("ta", "た", "ta", "た", "田", "nature", "N5", "rizière"),
        ("mori", "もり", "mo", "も", "森", "nature", "N5", "forêt"),
        ("rin", "りん", "ri", "り", "林", "nature", "N5", "bois"),
        ("ten", "てん", "te", "て", "天", "nature", "N5", "ciel"),
        ("ame", "あめ", "a", "あ", "雨", "nature", "N5", "pluie"),

        // =========================
        // DIVERS / OBJETS / LIEUX
        // =========================
        ("kuni", "くに", "ku", "く", "国", "lieux", "N5", "pays"),
        ("gai", "がい", "ga", "が", "外", "lieux", "N5", "extérieur"),
        ("machi", "まち", "ma", "ま", "町", "lieux", "N5", "ville"),
        ("mise", "みせ", "mi", "み", "店", "lieux", "N5", "magasin"),
        ("kuruma", "くるま", "ku", "く", "車", "transport", "N5", "voiture"),
        ("michi", "みち", "mi", "み", "道", "lieux", "N5", "route"),
        ("eki", "えき", "e", "え", "駅", "lieux", "N5", "gare"),

        // =========================
        // FAMILLE
        // =========================
        ("chichi", "ちち", "chi", "ち", "父", "famille", "N5", "père"),
        ("haha", "はは", "ha", "は", "母", "famille", "N5", "mère"),
        ("tomo", "とも", "to", "と", "友", "famille", "N5", "ami"),
        ("ie", "いえ", "i", "い", "家", "famille", "N5", "maison"),

    ];

    let mut kanji = Vec::new();

    for (rl, kl, rc, kc, k, g, n, s) in n5_kanji {
        kanji.push(FicheRevision::new_kanji(rl, kl, rc, kc, k, g, n, s));
    }

    (hiragana, katakana, kanji)
  }