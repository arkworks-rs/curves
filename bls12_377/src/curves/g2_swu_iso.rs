use ark_ec::{
    hashing::curve_maps::{swu::SWUParams, wb::IsogenyMap},
    models::{
        short_weierstrass::{Affine, SWCurveConfig},
        CurveConfig,
    },
};
use ark_ff::MontFp;

use crate::{g2, Fq2, Fr};

type G2Affine = Affine<SwuIsoParameters>;

#[derive(Clone, Default, PartialEq, Eq)]
pub struct SwuIsoParameters;

impl CurveConfig for SwuIsoParameters {
    type BaseField = Fq2;
    type ScalarField = Fr;

    /// COFACTOR =
    /// 7923214915284317143930293550643874566881017850177945424769256759165301436616933228209277966774092486467289478618404761412630691835764674559376407658497
    /// same as the original g2 curve
    /// sage: iso_G2.domain().order() == iso_G2.codomain().order()
    /// True
    #[rustfmt::skip]
    const COFACTOR: &'static [u64] = &[
        0x0000000000000001,
        0x452217cc90000000,
        0xa0f3622fba094800,
        0xd693e8c36676bd09,
        0x8c505634fae2e189,
        0xfbb36b00e1dcc40c,
        0xddd88d99a6f6a829,
        0x26ba558ae9562a,
    ];

    /// COFACTOR_INV = COFACTOR^{-1} mod r
    /// = 6764900296503390671038341982857278410319949526107311149686707033187604810669
    const COFACTOR_INV: Fr =
        MontFp!("6764900296503390671038341982857278410319949526107311149686707033187604810669");
}

// sage: E2p = iso_G2.domain()
// sage: r = 8444461749428370424248824938781546531375899335154063827935233455917409239041
// sage: E2p.order()/r
// 7923214915284317143930293550643874566881017850177945424769256759165301436616933228209277966774092486467289478618404761412630691835764674559376407658497
// sage: E2p
// Elliptic Curve defined by y^2 = x^3 +
// (69357795553467368835766998649443114298653120475771922004522583893765862042427351483161253261358624703462995261783*
// X2+203567575243095400658685394654545117908398249146024925306257919445062693445414588103741379252427065422417496933054)*
// x + (806998283981877041862626354975415285020485827233942100233224759047656510577433749137260740227904569833498998565*
// X2+249039961697346248294162904170316935273494032138504221215795383014884687447192317932476994472315647695087734549420)
// over Finite Field in X2 of size
// 258664426012969094010652733694893533536393512754914660539884262666720468348340822774968888139573360124440321458177^2
impl SWCurveConfig for SwuIsoParameters {
    /// COEFF_A =
    #[rustfmt::skip]
    const COEFF_A: Fq2 = Fq2::new(
                                    MontFp!("203567575243095400658685394654545117908398249146024925306257919445062693445414588103741379252427065422417496933054"),
                                    MontFp!("69357795553467368835766998649443114298653120475771922004522583893765862042427351483161253261358624703462995261783"),
    );

    /// COEFF_B =
    #[rustfmt::skip]
    const COEFF_B: Fq2 = Fq2::new(
        MontFp!("249039961697346248294162904170316935273494032138504221215795383014884687447192317932476994472315647695087734549420"),
        MontFp!("806998283981877041862626354975415285020485827233942100233224759047656510577433749137260740227904569833498998565"),
    );

    const GENERATOR: G2Affine = G2Affine::new_unchecked(G2_GENERATOR_X, G2_GENERATOR_Y);
}

const G2_GENERATOR_X: Fq2 = Fq2::new(
    MontFp!("44471777796618567688228760095584248343372454885978087674329841655595593880133139294404651664057692271364857231527"),
    MontFp!("152209914092745808277594956866181055187624831129109767937025242463317365117655129123148193049673425418513926319001"),
);
const G2_GENERATOR_Y: Fq2 = Fq2::new(
        MontFp!("115206687171448860889110309021279060303629519187879257051215751573842462972180856243991157572371972099444077110343"),
MontFp!("191377956145194479040228903677940355038998863371661730030204479850936075480341608934735952709786495341106477933498"),
);

impl SWUParams for SwuIsoParameters {
    // sage: F2.primitive_element()
    // X2 + 12
    const ZETA: Fq2 = Fq2::new(MontFp!("12"), MontFp!("1")); // arbitatry primitive root of unity (element)
}
pub const ISOGENY_MAP_TO_G2  : IsogenyMap<'_, SwuIsoParameters, g2::Parameters> = IsogenyMap {
    x_map_numerator : &[
        Fq2::new(
                   MontFp!("165752316658948679552567650341600213993620343632797226373648182250196112194084163699689918190990441453209217107673"), 
                   MontFp!("172182978063994664796636281648715261218877265445686511820228400278128135165425091257965367402286789184662480399420")), 
        Fq2::new(
                   MontFp!("49078863819486020728803126419770411403544927967564775122533948145670810135602221046611632159195633125363688522753"), 
                   MontFp!("133330677606878026253733532636681674371349711466687180056118155523679405609126901243884039337337134962627419965345")), 
        Fq2::new(
                   MontFp!("88440326308038176392218244342168484162310820452393341528824316035047758188693394849605113877264996124652991932266"), 
                   MontFp!("26460985441766134300772651139298255538590921173641559533448371120006177647448359485069994828122162245692248966113")), 
        Fq2::new(
                   MontFp!("240831597672798022181780196442988629902557797416422752447288392631352072879531084668083129009311685341499602842359"), 
                   MontFp!("124795068789978952575783920730487695370136831800946532752608526840944057283524691812795315973894625798220920082767")), 
        Fq2::new(
                   MontFp!("231856156439094824000656216233999389240375109059054378946071472571709648546048606758615235778059505184730503731408"), 
                   MontFp!("134026238756820071135251263743482298414233385640297868129438877114735051445009051866011097877494554014116371908672")), 
        Fq2::new(
                   MontFp!("161628978970526519329329822295337582413127505041035400390544637404697415598883543152357105789965717470570494458589"), 
                   MontFp!("116602947282570158568911982642223012726308726836613908383578383334370050655181609483409110122767402070015216413338")), 
        Fq2::new(
                   MontFp!("231615170079008089001496386178968115998492752668752266579314749799030868007672086089822600816657899022828734321418"), 
                   MontFp!("141639542381992520856560441410242716791591163086143661904501184306161835850893036003163803884002896297253767009574")), 
        Fq2::new(
                   MontFp!("96957224446676123824350918241672464825735454895610578698586352322390662445273628285471423071856275085141118045105"), 
                   MontFp!("97587419381711441517698658129839468394457401157021696651597713140291602428957555768352336399994179660880524139808")), 
        Fq2::new(
                   MontFp!("18662780664429933771192510151421557554668611953190652639195940454142648933454488952361465779870877163142807899993"), 
                   MontFp!("55517007457989983858891530398020104232682844055600438506715652841000901588962918721851719951820185832313468320365")), 
        Fq2::new(
                   MontFp!("188864327416940344326229259749844123042825201085435432627935318423081174922012610651352346391417830722940140585036"), 
                   MontFp!("233542978062801907184831603532041452683131033894029390005010898310417350737942550513935694209129350870817277172583")), 
        Fq2::new(
                   MontFp!("33702103741469458207888758659069786556456463428431771947788685622778743201883736580054112022350187936628758294279"), 
                   MontFp!("7531651998638745138624528632013700806848273210661661935097722938692087106885113623803921367479582718152259987647")), 
        Fq2::new(
                   MontFp!("129405180560592211762572081137246817341681076084973348177153469170177032429929182033872985192027066614650754524309"), 
                   MontFp!("71494892585734577638768956295356005795556178117525317604450863442514001295461407965931789120761632296953990284809")), 
        Fq2::new(
                   MontFp!("121805396188033590038927712795579559087103093135515082191363074395734682090746320669474388875423586929711608364404"), 
                   MontFp!("75932324143801627944771670190157701465835368459121478812142087933983158148741884038298732552979549182443318608554")), 
        Fq2::new(
                   MontFp!("205121513164720886728676669276499362266139599046368626660027529707299170205603076104821498121463983514654737906998"), 
                   MontFp!("168051662766288660516992486993594951443897767271245608184500932350745292043277998040372769419920145370014740823389")), 
        Fq2::new(
                   MontFp!("224387508661509885922784938707800296916307265709615639996851552571542396031861010679067114387017955365668256976459"), 
                   MontFp!("139097554917981907719834170888130444861465074977932740823078016609751284491153969166862091045221603146530147099779")), 
        Fq2::new(
                   MontFp!("142477190388553987083175296028785369398580520754966744245694134692905987471896623788336582063245356609279657681007"), 
                   MontFp!("140302880976816076816721836344737338071783799730513599397010923016385668972882234545823525686327074617973208860421")), 
        Fq2::new(
                   MontFp!("205538184807792915395400814312734290381492118179294333381188081024280220461983210331268164749465891791515156713363"), 
                   MontFp!("13030331252003455924520111292282390143014750833628314421882396445822808054003584720817672388894671968339344750750")), 
        Fq2::new(
                   MontFp!("139197805910452438551419010260519164074855817417063470117576949912038778696248952069759607969690314639425179841530"), 
                   MontFp!("134745690770497208213126494241047670387239583870118646766457825646403640347226342104549240830903829682059982326727")), 
        Fq2::new(
                   MontFp!("46999088780962505530452862218145506822520603893157196954987630663398814970386085186714696215299498862118704356116"), 
                   MontFp!("239220883081126775212690475926873251881459118214247205003758844846022330659109168499739536107988232402392010148915")), 
        Fq2::new(
                   MontFp!("137461512605659170682300574841422927080299499340556631216517745531687218204749421916211266297116812992348911035943"), 
                   MontFp!("126932856673577834557989832150639712812952235385146245135235966945536973066700604316981524138335070494135486842267")), 
        Fq2::new(
                   MontFp!("57815299880375466472857931022912171296473275308202666945592250229771936329503691945881925004852905385160277065937"), 
                   MontFp!("207900273391847649346738694548609379565855077833395139615718913929125517933139374225345876078131155102796477330196")), 
        Fq2::new(
                   MontFp!("182397511129719455005407008314265069548677727690813781407770284951663734172103638427690475141072553074575221965383"), 
                   MontFp!("225121846650282460844501132545123914298308844633699320249517374760159461135641190512758348530493533776082794775238")), 
        Fq2::new(
                   MontFp!("42757221749971324771094873984161893488770713619971906650868412147150411626107692517374735631529074208182971223761"), 
                   MontFp!("111424637101855455933266154646364284011426845669269128135151076357862608862363255550023430066507374024948118755170")), 
        Fq2::new(
                   MontFp!("257686488674545770403807165703608491821700153538449954828958424244540050698630649531963334687249831352703307010320"), 
                   MontFp!("0")),
    ],

    x_map_denominator : &[
        Fq2::new(
                   MontFp!("196537929755540830130458921156910352741196129560556501635658595085779576490417628044619830744899117989899096675116"), 
                   MontFp!("106967816747202586221026040614608875779671819314280336591550617355334247302203894951925800601923998790402234025494")), 
        Fq2::new(
                   MontFp!("73314120416427646620569455169905724114883313094893949291513517502168861559767103394033744003864213510722887906274"), 
                   MontFp!("38999017135204040984255776995893429123212353273299706702441986090800506456890730978953545316903022073202240280957")), 
        Fq2::new(
                   MontFp!("133779461364688439286044255858523747234865723284672664672066362220940987786797573428234566651244275384657571315397"), 
                   MontFp!("154931903368935230733381648548242132592387960818255334523314635613616976204585469590179605887364646535250639335453")), 
        Fq2::new(
                   MontFp!("247957910140234524214324761874381439955705875777136703199106095643204254634304448830280410483013931961038942096519"), 
                   MontFp!("214943806307523271117409515396321303330984956986022871981067208079182219051826091487389512723678609613943324945884")), 
        Fq2::new(
                   MontFp!("11697862001088266121450094179739500241088837734277696824118211258364151630931186792937914301859707394679202145393"), 
                   MontFp!("95980723944521770226526824868742386994509079773043937452565624851192578861364669763702851513923262074687274763858")), 
        Fq2::new(
                   MontFp!("168096269708683796856556357930292925811554548435612382636199127159818409693729567946366892866365435246772528367031"), 
                   MontFp!("99720174640078175171062115168656883367851695095484071724968247253018133737453004325770034298778522431209097310502")), 
        Fq2::new(
                   MontFp!("33059404918884325948584592996172619413923041143099424466021368531149134447772287601175965141235934645074697267050"), 
                   MontFp!("10757428905957703588038877674336794621171834192483169256644941002565404396356505770991807551250572677872221995215")), 
        Fq2::new(
                   MontFp!("142027935684179419855710336591935481541662612521926997142809731189880364304749483394981554800161483370077741056896"), 
                   MontFp!("1943403947563275150997369785095274118967148389261968103605246462390957897712088493386683316483490223770940902453")), 
        Fq2::new(
                   MontFp!("200535851812830711305923885193456283997079838967145939474743725430895019937175928830731575175640901984178880783011"), 
                   MontFp!("73874168720549156282270730246833500558176745413797996774310087510749148634278604002052708223696920208907646881989")), 
        Fq2::new(
                   MontFp!("220384543328043309613139993483671702409123249316603413540407457441864555087013622511692877380446192851630287225413"), 
                   MontFp!("39000405073743042346296304484168728185850505353133307908773204457381817815206498660334035187329579833299098854925")), 
        Fq2::new(
                   MontFp!("219195224293908756855578672234544437367397890301565855376597788842679778002659222115121213889017072727428356719234"), 
                   MontFp!("247894577734607804564008327202067464850944943412136922154651844411293409127507835027401641672218158657943826643185")), 
        Fq2::new(
                   MontFp!("146699001487357489560227247646638441970227213145065178169054120124066032784405371946823057532199624317631250110158"), 
                   MontFp!("196336324454181158449524835117699225596467237400207491815838252818724619960701247377784788803043096102210425517795")), 
        Fq2::new(
                   MontFp!("155939253194251956164424003889230633804182501306742152137449150503013281009623802843490858570697615902305132709111"), 
                   MontFp!("172261303485740204844677209985093962119870302741179905216184157502752680126595180139007438892219460422745105017475")), 
        Fq2::new(
                   MontFp!("137972103666241533333852948884443545062916813938500094392929132716673981519930321922556812217620174550912349461419"), 
                   MontFp!("47600282095791674213992406796226738606147801920837771594412659335039656256887529097801732359033209329929517773020")), 
        Fq2::new(
                   MontFp!("135098057022357227608956235549944366234286127511416070373827898662879730495316177891045223676768538262860577751087"), 
                   MontFp!("218641591773893348322227471378547165043111820723080862748702228639502320411481947789769811366015509814793754417785")), 
        Fq2::new(
                   MontFp!("228687493726193558146661770435566220015197012398911029567178581932152080915557441338298274247922585720118620741642"), 
                   MontFp!("48223421552324743764826987807666420223567068651197810526658625431719723647078135623842652870214605734395702563953")), 
        Fq2::new(
                   MontFp!("82401683815523491481199527201592079745651015539510634295850130611233688561451492330603949648060220533354045095181"), 
                   MontFp!("159070381032762485827712709726121404273458384870681735035622289092845201234785949112608113110663482448286550123895")), 
        Fq2::new(
                   MontFp!("61539107135026562717413992304341045078777699607129438878970720148394005607774781361280841945254066290447512808860"), 
                   MontFp!("258564647705165711537697112631909171171984598885182416737094411313452565619624851452883996847677936536536427515609")), 
        Fq2::new(
                   MontFp!("219867891253233227579149075701158020315633373195728794672342716359369374905471205886891783929214978430681990036959"), 
                   MontFp!("40524381030862708561992431051313657250449208728762250622105264621614810013551839533882876237430311293361821844681")), 
        Fq2::new(
                   MontFp!("88800087516399959501800534486349824688877578947555023348699585957763763180753947498274724426567091309571649023166"), 
                   MontFp!("122245180850560437899129167839042992977076962956306963014567407897293197200681367630073717776421484831646532593850")), 
        Fq2::new(
                   MontFp!("69204740140688189359361744597982172008615446130082862488352885921056331761951244674105352971861180999345144984246"), 
                   MontFp!("38216467720351249271557670250657907497353617320059247139049052120842234439257669911851800147147313339669901995490")), 
        Fq2::new(
                   MontFp!("114765242606519624982400506165904237893471895287563151339459173837887003905317760268941880935997925302483810508170"), 
                   MontFp!("226808321937551848279625259185874129283473963677740840941191767963773773116795416044456897499248110949601850478751")), 
        Fq2::new(
                   MontFp!("1"), 
                   MontFp!("0")),
    ],

    y_map_numerator : &[
        Fq2::new(
                   MontFp!("243169287995837894205750503657473181252400776697661357268613577074201794943537027717771587727860769419520957117060"), 
                   MontFp!("154445371651863854130996979206021232172872232688365227537444264835087932826365764405635125797374865965304745730822")), 
        Fq2::new(
                   MontFp!("109149004424675517113489432756837393820953128532207867425106578478986226345054217066591849467433110788215195319750"), 
                   MontFp!("30408441237651674477115309504276625429344634933425091999725383701660094027885762738289460271315609173544614563248")), 
        Fq2::new(
                   MontFp!("8414285408102090292522571401032945098403423241877066651551931468973120658567171350501434823318074450118610388181"), 
                   MontFp!("226047422399128874433860903177676209375847937545805175562415311468986239012130226120019081492247088609694678876810")), 
        Fq2::new(
                   MontFp!("228308803559737454633222698485074629499383737811342884536963429506633258142551503400414163815852158951494613610809"), 
                   MontFp!("220909287290837789818195731110558629271153823543746871132117978761339034747123501189473420274677281822601971748563")), 
        Fq2::new(
                   MontFp!("116623955280658732717402646061913268461869836841204913444259922610012147799771656282704605878638711580234302879520"), 
                   MontFp!("251345693404812657374633929641944735274020119374936409701199723189056283828393555325905238148261248120379910778583")), 
        Fq2::new(
                   MontFp!("142632909729670553826438094523500302011158161959746397893981306350515911350319381478896794266740222044724793183031"), 
                   MontFp!("135097007131619291616144192105571840182910366835929043414300810591005223344182310684819863256436016908585466099814")), 
        Fq2::new(
                   MontFp!("159262246805999098136860288248138175456624792734939305704793543040582454889431805248352849370505960249829264037333"), 
                   MontFp!("256411146327954053251262434650444473133439725697572806395735688747939610541453396276159230618136278790246160635595")), 
        Fq2::new(
                   MontFp!("106675525854808944323662773997719159035717496275254424840883415090817949089664218352587480756720528552217297479523"), 
                   MontFp!("142202207982399429498494980946602932891398916434890751210930378360132151108127041093945093575972538591541631204396")), 
        Fq2::new(
                   MontFp!("115853993705912938985758922127173369175321347209545356726288637524744651943071927137158115778405271676616612170666"), 
                   MontFp!("188439202506521797668192307957766105517906171778775206324453830870041256388075168650527562921934698697140423464142")), 
        Fq2::new(
                   MontFp!("199891426461397900698689228549412057991574595606781836622700872746783962617889812808189413859146835266670353365164"), 
                   MontFp!("123487321384490387195094801639396482206262484603737596281845841408384878196277195829349272799617445074531384638014")), 
        Fq2::new(
                   MontFp!("203453160391122297114764634999687500867096029894782931052056493086745424054313508850135956093450854351511962568248"), 
                   MontFp!("3933321808920817665892338621688661599151270488240879901971434149901647580182088988848276352998263499828820719486")), 
        Fq2::new(
                   MontFp!("216229669548325266866202681779047392389311278655704899819569794547799955366773265486059541504823551138048188296915"), 
                   MontFp!("41448968894064940344019909320065089603789758666259308674991068396259424208998703895462327945020441899649105710894")), 
        Fq2::new(
                   MontFp!("202678826482051686554967485240375873611017127444692775767942717540980994219310434688309713205309853725035377739266"), 
                   MontFp!("48778316120483961415587198479185523835826749642473845435889717611018677968038563827240090688089889339896065735651")), 
        Fq2::new(
                   MontFp!("43364741387169348753014627410136368149262698966106150910900756728904165177527487078077667350512959263803465594111"), 
                   MontFp!("61944739699039529393579599024212698483276675572994284564132452254474389809816373777333943401872462638231436446348")), 
        Fq2::new(
                   MontFp!("1902545032251691771730077590223241149624964994150687591044314892275508885163105731414463515832696686914784357054"), 
                   MontFp!("67221897212365550931740188657735915316732820967428518278153545138481072202717711417949297443415145931349647354864")), 
        Fq2::new(
                   MontFp!("232464396645736057215489125286424902556417590819993013568149210848680788030572385843387291711255018198764185991688"), 
                   MontFp!("41800154023275681622180037448850007404785328883356603798952195956734186941687720006035939799906615555216990599152")), 
        Fq2::new(
                   MontFp!("187038260664272653235271156369560372695631446985830424056061915935191103410794701043280599214000281588074544657045"), 
                   MontFp!("38290302770763423573829549940707041833171456153861823771988991461936307395626028842226881832323571911777783325552")), 
        Fq2::new(
                   MontFp!("8193038016485856982946817225511231096542148907426451941320763511467909442967073658628847889502238964737537651732"), 
                   MontFp!("9418692556935347898382092734571186686450013671235254851703843297990915553970523788207837029694342875454233715193")), 
        Fq2::new(
                   MontFp!("134073844001083825421215848942909782702386338984309026786345170296027964134133613977422644522116018820345983847098"), 
                   MontFp!("153830492090479629579603390014414329445124716355506854714467139884353350218676155251380590350715220577136073627555")), 
        Fq2::new(
                   MontFp!("24894203921911934571199160858232802417038583022586807490232139245280305064770051397858560092019807972764914216208"), 
                   MontFp!("120208242722200714489749801697072499732825359039071841003310452443348308205795253556381720202955786470886397257982")), 
        Fq2::new(
                   MontFp!("190392008574458975806418600277835706985376229847531539152452591238358119216217627352637258288556199770365835067627"), 
                   MontFp!("236947842470057836630333692287445445381482585314120394670322793497639860754696181540315462907276465780536189751938")), 
        Fq2::new(
                   MontFp!("128014602802339573117431114877417430852771121268703430430938496966993823548956133449208721198231566864014207876438"), 
                   MontFp!("76313913933214383311039506294052736258824720597935452573279680967713148986901401959316819572744945391130691484709")), 
        Fq2::new(
                   MontFp!("25945524144868616798377005434321968607597029473408456417628770501736226577600936996306306386148517051252174176056"), 
                   MontFp!("93302878136439028547402102681387844515310157832968167882878653011659204369510932686206718073261314562836132094987")), 
        Fq2::new(
                   MontFp!("89345438915485267000169594163110872264018138861479961667441187849751112704236404111089533981719810422892295971197"), 
                   MontFp!("225199447663521472758596124691205483139271667363437613911741821442536429098852529066869544898685674003162780468715")), 
        Fq2::new(
                   MontFp!("133226465537371725791207823007732608016851433266603356824246032571600774858733596680855277271698121233692296984412"), 
                   MontFp!("214026235442364760645768878901893433888530027239186932434340221483611429797860448445573321733516533800376783586849")), 
        Fq2::new(
                   MontFp!("209865017468509971341642462085093919192185569139223034709204939485243056860760867821924437786503336074163109167043"), 
                   MontFp!("219490420378012040044597900078465204875085141304274262719756064241884227366143829466191943246131284770521917871841")), 
        Fq2::new(
                   MontFp!("107794270350250727824303719264349327362253764840855494366195678657690526275364378542854833701549538802096418248084"), 
                   MontFp!("214497132288922282410470995366492104127705853917251639956391350188219443322307620708083843529019036699996096662829")), 
        Fq2::new(
                   MontFp!("216436913923048926393371382639334167145590308917722616640273699755872914758105310937706004925121569777096571773501"), 
                   MontFp!("14821532235517245029225575994881495294340097494060025842437989195232333244802578196149414170959605497405878582540")), 
        Fq2::new(
                   MontFp!("149340524242957423974772893433814190392014381473852786295383636551096665463613496468840974015807625484823068784839"), 
                   MontFp!("92602205576740019970555092291786069878442230185393269119060098961688837594116147683652993589116622567477525635445")), 
        Fq2::new(
                   MontFp!("187289608720372854303118076618428364941868395899689208696722069999084187290922488892094161492110977423619035272649"), 
                   MontFp!("138246251209835932037747211155451146889753321830881007441732932302281412878493276648902633332871835811473542877960")), 
        Fq2::new(
                   MontFp!("165653628641840315664303139225783620502451401675361521932235743336920154286645843675719999005591028855021909439672"), 
                   MontFp!("221484407095875062109245088271905042727631591858266177514520864715688286361376419124935364810076972840743950061836")), 
        Fq2::new(
                   MontFp!("133980604703672698089766182661998480874540278232523164821108522954758888202993741126021280431373622479768313949356"), 
                   MontFp!("38886256490758184304393553179653915986060051480245869194662478974097783410818598716423697164666847852998235320966")), 
        Fq2::new(
                   MontFp!("8411654157888762354868203383638678565276209861191964793314989111047210939710084789719415109438273538021505111510"), 
                   MontFp!("187207294428708203829145346569036650547801585764458185253951079008883539428999915118458145884040644479498579194069")), 
        Fq2::new(
                   MontFp!("191144230647045707590184821948778479495825928592047153194222027257046414968351470170933284561757547536684715232224"), 
                   MontFp!("0")),
    ],

    y_map_denominator : &[
        Fq2::new(
                   MontFp!("177304823246185962354212404236288831041380791662214394697399928748373114098169675564032904690251242875327747673679"), 
                   MontFp!("234106598974619695004693596968258258794247055108931498762994964636371479098512727352039267846913406623802246782945")), 
        Fq2::new(
                   MontFp!("255874157960252694683645508260848371559149621054025374393554376526882940742514793344046680765937904153005134511200"), 
                   MontFp!("19068358873460915055376626440913257473060029137260026174266944920186136734103362122730468957229595374427187617425")), 
        Fq2::new(
                   MontFp!("83946178094995839681455048029661822915614318352963730526672352524233381944447759416757234629624830143848209247040"), 
                   MontFp!("36167189440196390196320129647971031300455228428629866775570898825049060037811108115927676106354625467897211624573")), 
        Fq2::new(
                   MontFp!("214137118009637944275213937166601531498145257806838837034827533674793291112410824873653425491233537265355337125438"), 
                   MontFp!("75310151275642225944994533227518963961512910025529659163648069668626195571780520556481029340507362571133885889206")), 
        Fq2::new(
                   MontFp!("107140053800026140817203074526089346919722280052456645787788712045148531978814339001150298053597504647766497707422"), 
                   MontFp!("114397460921328140828185121166450637136573513025702964340164304518654108540682577087979875141067314446917571826582")), 
        Fq2::new(
                   MontFp!("42398151340378868438123040588425908782227436520374323184618125390984068793920538080412999034838310511981244418952"), 
                   MontFp!("82194329196840936158921467961561828669469444994699107543787160001285217807552090667641931153824104285439311091477")), 
        Fq2::new(
                   MontFp!("32044478312863504453978511975839237915357713065285858656377527520925624222082496835678894223326750023304346513708"), 
                   MontFp!("224732340440722096332247540469311391766792864305974461767563394934556509366444399113606207665094042937608880394380")), 
        Fq2::new(
                   MontFp!("53290030879673160724264978063216325707183784347799183367929912851157629196486013218134779085284663628473135140615"), 
                   MontFp!("42967590874964323361920860309631969474991176192252393149421408476343364383848642144439727879920981862569415477634")), 
        Fq2::new(
                   MontFp!("250488593850017034090300371968459931740406675270099747930030660805939980644430804509506597720682995099971419762057"), 
                   MontFp!("131736273443849165304852712527070616520885505701320868662058330388440710419459541866884603770735894010610491333882")), 
        Fq2::new(
                   MontFp!("251064692215092635541196206457923985861467397867989080397969990645886126152674833092283559946890888232618697517884"), 
                   MontFp!("171430383391121065822039978510482289343958135960126315509375831831735263180943544895493884988847947752888720502133")), 
        Fq2::new(
                   MontFp!("255810123836950778051032251049468471118744836441263107437005316697409810175422976454215094737538073112171964000966"), 
                   MontFp!("78363344687446114757879143124020926645520829625790622079288990711202726499291972408305265373741655103522048633806")), 
        Fq2::new(
                   MontFp!("94953611600133917480746113251669332369937157892937924494319507354263981756523698288726321437604686331762128114942"), 
                   MontFp!("224037954053161681052577381077631881138732983068288196649494577229506443999757164090758954620762136536790092765707")), 
        Fq2::new(
                   MontFp!("18177910449767953614338723180992575758462819793657888834925741153507922227776503487306285172452430778418239589878"), 
                   MontFp!("55976309667093193926953058482403983650044130588205371047077237394269232758375928755579870418413040530924659710422")), 
        Fq2::new(
                   MontFp!("108398849957050915959578499238335172000970311919637037913625633347326921657585237193100994767528244599176367431882"), 
                   MontFp!("204809230842263072415312635210643987712012160534293694062738501600937603588267937911969298058204043106501986725326")), 
        Fq2::new(
                   MontFp!("40836628940164991036725499428168139451294386215534361893839871958837737306822281294361671587764896498700322394958"), 
                   MontFp!("119396884503349014053839666170414789560955868303656326650021743484252346209353246139311353657707899076368015332219")), 
        Fq2::new(
                   MontFp!("57449149099548285338146473529383255206310079371370663992648410210841833627207062160011080280732475904935527767063"), 
                   MontFp!("53213606042373287683153647684084583002441527525177758051678463570615020765660540938870067881706148841340293404257")), 
        Fq2::new(
                   MontFp!("136011650568921952309089811450436645471254909718073766303847394446918967077385999380499048535832357755732371368738"), 
                   MontFp!("167478505497067957490757520193067128323382872103395718848436244112857102765738573826312783708349214669979612036158")), 
        Fq2::new(
                   MontFp!("124201488690791095020042847186337439989685011854729481246272047846908500431421470344897520044190299684207452017073"), 
                   MontFp!("206045464105062318563700338460670926332476051084269665075012377410704353049241671458557389593362582538704238377616")), 
        Fq2::new(
                   MontFp!("141307886851791570111930048284226290849958071383456038631306928334570275405889250568493573543543795501685612269615"), 
                   MontFp!("33695298953151791677564491610276872092952551110021466172491397823758283921068636555353240048692541883780027233962")), 
        Fq2::new(
                   MontFp!("248265339860070148327157063205450906034372346050742532250909040050681913704941472496789561564874787781299103889328"), 
                   MontFp!("246735083688655178976599901436968434779493759990448799231310864308891009496294971885106650388027197609829556319894")), 
        Fq2::new(
                   MontFp!("242374981274128257174433646391488180576863913259197526647513270422870835725158808599624055920155693832356361192562"), 
                   MontFp!("228823832066259533984346839854456963456061412139464585495219844904598927917971403987214543635705786807302526573779")), 
        Fq2::new(
                   MontFp!("252218794556637183364789705825049189219556217519670190016584589877973440046635828866291293002549365274213665103495"), 
                   MontFp!("246226868929550107779875102413192686259972538740042993595504829761267272514998892312681511407413880843390092913913")), 
        Fq2::new(
                   MontFp!("194440756770673250653849096711805826346541581797630354589253365569405779009028682866443842541588052010949210024484"), 
                   MontFp!("75265817091612360444217709043194311507703596250641164601263089234422086318666070167810083868284770450890779266774")), 
        Fq2::new(
                   MontFp!("30126025128311053094362231518416999154688680759401769180563223025361877725315179695978985780460875616652938854146"), 
                   MontFp!("53390417057360854696711134553867053334956120915983753729596602193155006833056717338565729257476036879055975910097")), 
        Fq2::new(
                   MontFp!("101332532133743769759178027285109552486478973958553103055307446365529747250973790438239230671819476569086244438727"), 
                   MontFp!("148745905560783494991892238622790396213255789465409808038709750840775025617296316699696997331989504328034553213894")), 
        Fq2::new(
                   MontFp!("64805743514968884017432304617184871899075979363892814607985017391426869625002757534871594116135107848421987411961"), 
                   MontFp!("6046324386958113626500748531301314307229746408846776295678625368971798823548167403538529285882066298168231669149")), 
        Fq2::new(
                   MontFp!("42585898388361518080924198789209195881860509830112816215046001795045034138585747974649960102171072364466234418812"), 
                   MontFp!("132695208880203057798268960407754614753345990251279710979596670783892175104478291787481794529190706739877760089560")), 
        Fq2::new(
                   MontFp!("212862020601301354783026588297306531610140885387724836359356800753780855161155144746080869191538655229274242914307"), 
                   MontFp!("157598266513011182093451150345114371606842593014102635902061162138398777778450012845089379844999350325670077782200")), 
        Fq2::new(
                   MontFp!("4121090928751590306336774011079865996138664888804166805010515676212588791761189056383637824656513752764121888675"), 
                   MontFp!("183219245849642047090141657656072708461001592401873575143115550847958265143529306190408825011434780265059908079650")), 
        Fq2::new(
                   MontFp!("79081485025787885179619178550309706744599846607596447880706626420512740963985633088538634502003788677250834958908"), 
                   MontFp!("183808890703436998546164599111050972902808423546263572335175957576564587842514119276068990911067829699066843757937")), 
        Fq2::new(
                   MontFp!("212106075999882114389916784897163150756429321557076536924307653148222968665985018997130331497354557069316538670523"), 
                   MontFp!("6765896997590927451499527318422027932088882822980873934837948553969718709492787823596776070132570199077127482065")), 
        Fq2::new(
                   MontFp!("234716866510887739589745422464313597883163631119309437461957606368046054279070563732715561665778636277317684644515"), 
                   MontFp!("97451989647642778641795555357790293895920858058713680518946629728091416392679362160653023502048556793761866531581")), 
        Fq2::new(
                   MontFp!("172147863909779437473600759248856356840207842931344727009188760756830505857976640403412821403996887953725715762255"), 
                   MontFp!("210880269899843225414111521931364427157014189139153931141845520612300425501022712679200902179085486362182614989038")), 
        Fq2::new(
                   MontFp!("1"), 
                   MontFp!("0")),
    ],
};

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_gen() {
        let gen: G2Affine = SwuIsoParameters::GENERATOR;
        assert!(gen.is_on_curve());
        assert!(gen.is_in_correct_subgroup_assuming_on_curve());
    }
}
