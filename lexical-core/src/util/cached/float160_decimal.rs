//! Cached exponents for decimal values with 160-bit extended floats.

use super::ModeratePathPowers;

// LOW-LEVEL
// ---------

// BASE10

const BASE10_SMALL_MANTISSA: [u128; 35] = [
    170141183460469231731687303715884105728, // 10^0
    212676479325586539664609129644855132160, // 10^1
    265845599156983174580761412056068915200, // 10^2
    332306998946228968225951765070086144000, // 10^3
    207691874341393105141219853168803840000, // 10^4
    259614842926741381426524816461004800000, // 10^5
    324518553658426726783156020576256000000, // 10^6
    202824096036516704239472512860160000000, // 10^7
    253530120045645880299340641075200000000, // 10^8
    316912650057057350374175801344000000000, // 10^9
    198070406285660843983859875840000000000, // 10^10
    247588007857076054979824844800000000000, // 10^11
    309485009821345068724781056000000000000, // 10^12
    193428131138340667952988160000000000000, // 10^13
    241785163922925834941235200000000000000, // 10^14
    302231454903657293676544000000000000000, // 10^15
    188894659314785808547840000000000000000, // 10^16
    236118324143482260684800000000000000000, // 10^17
    295147905179352825856000000000000000000, // 10^18
    184467440737095516160000000000000000000, // 10^19
    230584300921369395200000000000000000000, // 10^20
    288230376151711744000000000000000000000, // 10^21
    180143985094819840000000000000000000000, // 10^22
    225179981368524800000000000000000000000, // 10^23
    281474976710656000000000000000000000000, // 10^24
    175921860444160000000000000000000000000, // 10^25
    219902325555200000000000000000000000000, // 10^26
    274877906944000000000000000000000000000, // 10^27
    171798691840000000000000000000000000000, // 10^28
    214748364800000000000000000000000000000, // 10^29
    268435456000000000000000000000000000000, // 10^30
    335544320000000000000000000000000000000, // 10^31
    209715200000000000000000000000000000000, // 10^32
    262144000000000000000000000000000000000, // 10^33
    327680000000000000000000000000000000000, // 10^34
];
const BASE10_LARGE_MANTISSA: [u128; 284] = [
    286119432080757541408869572143325302487, // 10^-5005
    172201869348562774237764357736372983589, // 10^-4970
    207280460411159723777029324213641809608, // 10^-4935
    249504778495140931061313034335657841029, // 10^-4900
    300330452607185233358206994840344586560, // 10^-4865
    180754816214859259216634330660716477758, // 10^-4830
    217575695712755581576194737224748223089, // 10^-4795
    261897217215050942152690595256879476969, // 10^-4760
    315247308116346805811584775614859822349, // 10^-4725
    189732572058981752963537067424138651391, // 10^-4690
    228382276221251216609907114219733998177, // 10^-4655
    274905165338640432981782799919604540395, // 10^-4620
    330905056120257278961339867723089711689, // 10^-4585
    199156236353481956319646797449847777436, // 10^-4550
    239725599502895439634666471219960793015, // 10^-4515
    288559194074255118072692509223241964363, // 10^-4480
    173670247686204924083107421313210697519, // 10^-4445
    209047956542506328453196169087216947529, // 10^-4410
    251632324573859069099285092447789160274, // 10^-4375
    302891393045352049173184566549975177119, // 10^-4340
    182296126176002271404258438754217314733, // 10^-4305
    219430980092597865398775428808211017685, // 10^-4270
    264130434554108540303155967169890995680, // 10^-4235
    317935445707356687586615805271283296184, // 10^-4200
    191350436021843909139667906379379661100, // 10^-4165
    230329709128764482519783597263513311152, // 10^-4130
    277249302433180993754455116767313576955, // 10^-4095
    333726708510335128237668597462122220981, // 10^-4060
    200854456613076563626740380571190591308, // 10^-4025
    241769757784219389187512594604084339659, // 10^-3990
    291019760102423206687706059722700680362, // 10^-3955
    175151146996764576806175644475313595547, // 10^-3920
    210830524246774608381579173630310697644, // 10^-3885
    253778012398576501004934697161380247082, // 10^-3850
    305474170815933557111351964617149844652, // 10^-3815
    183850579003521219086769513610577682720, // 10^-3780
    221302084622382958171161091825785589183, // 10^-3745
    266382694705978358311845816926252164378, // 10^-3710
    320646505250504328820353011062317385566, // 10^-3675
    192982095632832915927136629614810160062, // 10^-3640
    232293747943671330474885829268234866089, // 10^-3605
    279613428161660942687775155163925536189, // 10^-3570
    336572421343320016673588247209714562031, // 10^-3535
    202567157725005510368824826066831632525, // 10^-3500
    243831346757500185066930973484248304510, // 10^-3465
    293501307562835720355857844659769718035, // 10^-3430
    176644674047522876435405187328679556331, // 10^-3395
    212628292040404461121591575665297806863, // 10^-3360
    255941996665330692248349166751957836078, // 10^-3325
    308078972127511527671808710843323959544, // 10^-3290
    185418286767629706540227118453718225106, // 10^-3255
    223189144201723518861498177751230874609, // 10^-3220
    268654160050162587520122630767954439419, // 10^-3185
    323380682203005745135991760413494304977, // 10^-3150
    194627668528481639280044974268966674096, // 10^-3115
    234274534265797510746604947779427849929, // 10^-3080
    281997712968670612570785187184022523715, // 10^-3045
    339442399784424682227597493943672249425, // 10^-3010
    204294463168589658612095197458799112583, // 10^-2975
    245910515055563802805920940221123286109, // 10^-2940
    296004015365749074812131572210487601421, // 10^-2905
    178150936516173632052853442229332741253, // 10^-2870
    214441389535705167592744064145496427833, // 10^-2835
    258124433389263896143607651616633928228, // 10^-2800
    310705984776482363036976663133312325503, // 10^-2765
    186999362494172508445409680851355509841, // 10^-2730
    225092294880531385733101457570771069236, // 10^-2695
    270944994350785690774740498082721868736, // 10^-2660
    326138173688755386130294316007363276075, // 10^-2625
    196287273348418540044406250359247046545, // 10^-2590
    236272210902401175659619915337746664850, // 10^-2555
    284402328752194223028674498667984613958, // 10^-2520
    171168425374160558818812875131234577261, // 10^-2485
    206036497476067359568085344493177783949, // 10^-2450
    248007412578639543758750788750554161753, // 10^-2415
    298528063947000946300982383810765475155, // 10^-2380
    179670042998586475306582642079355280092, // 10^-2345
    216269947450199953022203726001299387708, // 10^-2310
    260325479915871259940065720815374626735, // 10^-2275
    313355398160596512284380923372171536666, // 10^-2240
    188593920172774309008396023667191486016, // 10^-2205
    227011673868826264270283822469891573722, // 10^-2170
    273255362768401181047535823795617750446, // 10^-2135
    328919178512538028180098172661194360752, // 10^-2100
    197961029743921145808919077817184704002, // 10^-2065
    238286921878468746820571551162559150237, // 10^-2030
    286827448876003079897240977949411079056, // 10^-1995
    172627991457029170767403061486316204036, // 10^-1960
    207793386241572760497271111760062759547, // 10^-1925
    250122190505167278923175314151349550792, // 10^-1890
    301073635281019018134776056791568918102, // 10^-1855
    181202103016636214953110628867301828717, // 10^-1820
    218114097616050229668875396804499931932, // 10^-1785
    262545294932344844363281491950409810603, // 10^-1750
    316027403292613338213889573744451485443, // 10^-1715
    190202074765057925994690657888451851648, // 10^-1680
    228947419546628054526484265270079130656, // 10^-1645
    275585431871899077347577613126954188917, // 10^-1610
    331723897174361853131491720259011441331, // 10^-1575
    199649058386542458855824248947815702976, // 10^-1540
    240318812447098575983722135095723711580, // 10^-1505
    289273248182154452343850939804906973935, // 10^-1470
    174100003370053687258760534141258406240, // 10^-1435
    209565256130190670313318477038881109719, // 10^-1400
    252255001302696846946450507552830400188, // 10^-1365
    303640912893940653816025398573372854582, // 10^-1330
    182747227026098952897000682305384965328, // 10^-1295
    219973972989560200776528160568668989171, // 10^-1260
    264784038479014374466807894794486107999, // 10^-1225
    318722192814072420255051486424751093316, // 10^-1190
    191823942212932612179995309886957891748, // 10^-1155
    230899671473933531094361039014687556069, // 10^-1120
    277935369650514896408649200822882732420, // 10^-1085
    334552531883913746232825784013178923915, // 10^-1050
    201351480976810922143475303777684812211, // 10^-1015
    242368029099973148176569777542121720919, // 10^-980
    291739903003597028857018076064012891551, // 10^-945
    175584567239767252436009176349603974315, // 10^-910
    211352234887088636709795629332968905898, // 10^-875
    254405998738880392120826394278324941591, // 10^-840
    306230081876844442602606010040453188389, // 10^-805
    184305526424615531007915636184963348490, // 10^-770
    221849707660762511002232690173506830912, // 10^-735
    267041871960885546630040052898199934977, // 10^-700
    321439961009182286313775737285650023301, // 10^-665
    193459639446953031773047832279434098941, // 10^-630
    232868570400778094979024381518929039566, // 10^-595
    280305345525941045955281543256270015630, // 10^-560
    337405286575137855484278275127554174128, // 10^-525
    203068420253004570555511362849258201390, // 10^-490
    244434719577920582091951313128332647752, // 10^-455
    294227591176883860910658765384315687611, // 10^-420
    177081790097651968937548429995999258430, // 10^-385
    213154451346726893197828921904416471830, // 10^-350
    256575337892558434874823357106039456778, // 10^-315
    308841328899094571460716776609676066664, // 10^-280
    185877113559722882849757812268737570016, // 10^-245
    223741436863085634409521749481834675708, // 10^-210
    269318958159276723570738682003462587676, // 10^-175
    324180903818827574883781864350871964922, // 10^-140
    195109284394749514461349826862072894109, // 10^-105
    234854258277383322788948059678933702737, // 10^-70
    282695530364541492733327600118866962532, // 10^-35
    170141183460469231731687303715884105728, // 10^0
    204800000000000000000000000000000000000, // 10^35
    246519032881566189191165176650870696772, // 10^70
    296736492054993710858538820923811161069, // 10^105
    178591779887855465971216179422709524914, // 10^140
    214972035442146840057310898846407268146, // 10^175
    258763175164940474024358370140027266101, // 10^210
    311474842221798985484709462972023293421, // 10^245
    187462101736953869352205554703508169192, // 10^280
    225649296983103697008657760146718317346, // 10^315
    271615461243554856334256923502490730495, // 10^350
    326945218854695637286627726376451846486, // 10^385
    196772995989530194869453349330805553038, // 10^420
    236856878264391044844763517461616138981, // 10^455
    285106096489670585936790172741528654512, // 10^490
    171591990174004115824164697706874025894, // 10^525
    206546345058196792336004061807532406770, // 10^560
    248621119282074859455540713067277593043, // 10^595
    299266786520261728429964812849422362099, // 10^630
    180114645474973267146862540974467867862, // 10^665
    216805118214338728866268952408480869448, // 10^700
    260969668290880926863172591227493900069, // 10^735
    314130811711382307066633098267742456760, // 10^770
    189060605228006182392392312888603654778, // 10^805
    227573425570369436769612731512559670492, // 10^840
    273931546782971479375390563391696876809, // 10^875
    329733105413523598515306621137323748265, // 10^910
    198450894178655650402145460387414827185, // 10^945
    238876574743184690044550569962995574899, // 10^980
    287537217694096924171828647053625243375, // 10^1015
    173055168025891443149980703111577662428, // 10^1050
    208307581332518039545108244048827174219, // 10^1085
    250741130331985048272140772152419092677, // 10^1120
    301818656997420388079954843762460942159, // 10^1155
    181650496651897519146841059375049677536, // 10^1190
    218653831821689225983283831391627738607, // 10^1225
    263194976350251220045512085810261627653, // 10^1260
    316809428853274492372013900068294130495, // 10^1295
    190672739278980905234011268012900725809, // 10^1330
    229513961347331010027265544850205633221, // 10^1365
    276267381758599632063665900974053670755, // 10^1400
    332544764491466903841018450837187271285, // 10^1435
    200143099932286655671730787308624406853, // 10^1470
    240913493326298641615844138681696116952, // 10^1505
    289989069252533161672114796900108597368, // 10^1540
    174530822505761712039447731217237692992, // 10^1575
    210083835801487616051911128195003264712, // 10^1610
    252879218876135145543382347991809205451, // 10^1645
    304392287466751627396517354913371243322, // 10^1680
    183199444147732647098291113971398213962, // 10^1715
    220518309549509536116819878254862307096, // 10^1750
    265439259779408851146124636128870317794, // 10^1785
    319510886765716212022616798893582840606, // 10^1820
    192298620118691323793126434006160815476, // 10^1855
    231471044219333359243331711099562723424, // 10^1890
    278623134575372567219295793757959387084, // 10^1925
    335380398798590387101387201551099249914, // 10^1960
    201849735252105676980043501206402713041, // 10^1995
    242967780867916354229831071348903668805, // 10^2030
    292461827934272657117530140389190165109, // 10^2065
    176019060002763462384229605132480020457, // 10^2100
    211875236526384853630741816527635933117, // 10^2135
    255035539062683014770771574381844042649, // 10^2170
    306987863477351134784845320963846824944, // 10^2205
    184761599635778507336249300908592424647, // 10^2240
    222398685819644772923408724432909219268, // 10^2275
    267702680382764247773857406001216956249, // 10^2310
    322235380213681951302311777413012241285, // 10^2345
    193938364967042587764792054637272501716, // 10^2380
    233444815284704863186097106861733483662, // 10^2415
    280998975074225114625497217882908229374, // 10^2450
    338240212773482905421750301799088019772, // 10^2485
    203570923180112735642201248436986911855, // 10^2520
    245039585474457989352572102319552871507, // 10^2555
    294955672015933876111147966464119066994, // 10^2590
    177519987813233533872886105190205630693, // 10^2625
    213681912660477278916296209718942792647, // 10^2660
    257210246354219496580154770123770173598, // 10^2695
    309605572160505742733775059528702430577, // 10^2730
    186337075741581613112936057639999011839, // 10^2765
    224295096200165269434233960510245848270, // 10^2800
    269985401344446258165976723822443758818, // 10^2835
    324983105622921833689303947065906723816, // 10^2870
    195592092043482825752591119506329293952, // 10^2905
    235435416844929996737569375415845609308, // 10^2940
    283395074544338574863902122593323954186, // 10^2975
    170562206298998297014910696924238505445, // 10^3010
    205306787807496274300827891046112368612, // 10^3045
    247129056515258332162012856568415153107, // 10^3080
    297470781294313466138077414803050375060, // 10^3115
    179033714148432729097339079120204735610, // 10^3150
    215503994458332079163908674147175469438, // 10^3185
    259403497538976677930370728426434131261, // 10^3220
    312245602243184891124346776009125656012, // 10^3255
    187925986051055013716367676406827977382, // 10^3290
    226207677415140527017218331718361546646, // 10^3325
    272287587240067114301290753821946287625, // 10^3360
    327754261094123180077492440215525305644, // 10^3395
    197259920575526323694769200993307030004, // 10^3430
    237442992414908733738631944843841719638, // 10^3465
    285811605735490026283941009870459553721, // 10^3500
    172016603106055901904334486123793461529, // 10^3535
    207057454283579665786905970243500396929, // 10^3570
    249236344633335759868509785791528294400, // 10^3605
    300007337099348930664624988819361041019, // 10^3640
    180560348142347439242531190593634637570, // 10^3675
    217341613285206967581186106985703835457, // 10^3710
    261615450742131735079439477001995836043, // 10^3745
    314908144061647133264325792279170517719, // 10^3780
    189528445118667412411096947676376026294, // 10^3815
    228136567354496507539643202190194397726, // 10^3850
    274609404048587715741751021453543031262, // 10^3885
    330549046417192824682216906383535227497, // 10^3920
    198941970807349381076132980368824080000, // 10^3955
    239467686733303432536633030679941851682, // 10^3990
    288248742870506935444873673344389974131, // 10^4025
    173483401664464396647103488143509244359, // 10^4060
    208823048824844009536963464377510096603, // 10^4095
    251361601756253037841061320713210493432, // 10^4130
    302565522307191836937915474253219426885, // 10^4165
    182099999859557794827203011016380459322, // 10^4200
    219194901626521125277142444314358466055, // 10^4235
    263846265437207165272931879528592391256, // 10^4270
    317593389575162665626692090710239950350, // 10^4305
    191144568475702113605744834416097546626, // 10^4340
    230081905083956979405061927775538180340, // 10^4375
    276951019164284089644956688178198325663, // 10^4410
    333367663085661217331754036989419771193, // 10^4445
    200638364008459464654678359795989951543, // 10^4480
    241509645772973950207016434782612372233, // 10^4515
    290706661657827968961126609804959690245, // 10^4550
    174962707724893627980956524469322824913, // 10^4585
    210603698724027866076345848756647137717, // 10^4620
    253504981107070726561874602682320251313, // 10^4655
    305145521353392500015179106554940781379, // 10^4690
    183652780303172908759920616718398576474, // 10^4725
    221063993097406902648195179701514468883, // 10^4760
    266096102457568229068429484807419331093, // 10^4795
    320301532379852870634979319798473338442, // 10^4830
    192774472638586394686410697807778237172, // 10^4865
    232043830855069634198237584007327880426, // 10^4900
];
const BASE10_SMALL_INT_POWERS: [u128; 35] = [
    1,
    10,
    100,
    1000,
    10000,
    100000,
    1000000,
    10000000,
    100000000,
    1000000000,
    10000000000,
    100000000000,
    1000000000000,
    10000000000000,
    100000000000000,
    1000000000000000,
    10000000000000000,
    100000000000000000,
    1000000000000000000,
    10000000000000000000,
    100000000000000000000,
    1000000000000000000000,
    10000000000000000000000,
    100000000000000000000000,
    1000000000000000000000000,
    10000000000000000000000000,
    100000000000000000000000000,
    1000000000000000000000000000,
    10000000000000000000000000000,
    100000000000000000000000000000,
    1000000000000000000000000000000,
    10000000000000000000000000000000,
    100000000000000000000000000000000,
    1000000000000000000000000000000000,
    10000000000000000000000000000000000,
];
const BASE10_STEP: i32 = 35;
const BASE10_BIAS: i32 = 5005;
const BASE10_LOG2_MULT: i64 = 14267572528;
const BASE10_LOG2_SHIFT: i32 = 32;

// HIGH LEVEL
// ----------

pub(crate) const BASE10_POWERS: ModeratePathPowers<u128> = ModeratePathPowers {
    small: &BASE10_SMALL_MANTISSA,
    large: &BASE10_LARGE_MANTISSA,
    small_int: &BASE10_SMALL_INT_POWERS,
    step: BASE10_STEP,
    bias: BASE10_BIAS,
    log2: BASE10_LOG2_MULT,
    log2_shift: BASE10_LOG2_SHIFT,
};
