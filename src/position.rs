// SPDX-License-Identifier: GPL-3.0-or-later

use std;
use std::sync::Arc;

use bitboard::*;
use material;
use movegen::*;
use movepick::*;
use pawns;
use psqt;
use search;
use tb;
use threads::ThreadCtrl;
use types::*;
use uci;

pub mod zobrist {
    use types::*;

    const PSQ: [[Key; 64]; 16] = [
        [Key(0); 64],
        [
            Key(591679071752537765), Key(11781298203991720739), Key(17774509420834274491), Key(93833316982319649), Key(5077288827755375591), Key(12650468822090308278), Key(7282142511083249914), Key(10536503665313592279), Key(4539792784031873725), Key(2841870292508388689), Key(15413206348252250872), Key(7678569077154129441), Key(13346546310876667408), Key(18288271767696598454), Key(10369369943721775254), Key(18081987910875800766), Key(5538285989180528017), Key(1561342000895978098), Key(344529452680813775), Key(12666574946949763448), Key(11485456468243178719), Key(7930595158480463155), Key(14302725423041560508), Key(14331261293281981139), Key(4456874005134181239), Key(2824504039224593559), Key(10380971965294849792), Key(15120440200421969569), Key(2459658218254782268), Key(3478717432759217624), Key(3378985187684316967), Key(9696037458963191704), Key(13098241107727776933), Key(16711523013166202616), Key(10079083771611825891), Key(14137347994420603547), Key(4791805899784156187), Key(6078389034317276724), Key(5994547221653596060), Key(16213379374441749196), Key(4600174966381648954), Key(2382793282151591793), Key(5441064086789571698), Key(13211067155709920737), Key(8095577678192451481), Key(12870220845239618167), Key(18366225606586112739), Key(1482740430229529117), Key(18398763828894394702), Key(12894175299039183743), Key(5973205243991449651), Key(16073805277627490771), Key(11840382123049768615), Key(16782637305176790952), Key(16565939816889406374), Key(7611013259146743987), Key(4325631834421711187), Key(7084652077183601842), Key(14113904950837697704), Key(6952439085241219742), Key(11697893679396085013), Key(15932411745698688381), Key(333938476871428781), Key(10094356940478519713), ],
        [
            Key(8854028305631117351), Key(18264149368209609558), Key(18152850504025660547), Key(445125824226036916), Key(7445032221575161576), Key(5887372625995221418), Key(12579614965563241976), Key(15542129933905340102), Key(4278411582816540073), Key(7817987688731403418), Key(16765308846548980593), Key(15594655397588023405), Key(11116801254932199266), Key(11592572287770353464), Key(10698558469286656858), Key(263236209937302172), Key(15461982991340303336), Key(3043744698521235658), Key(1070442759222213040), Key(650534245804607543), Key(5943000432800778858), Key(26206987068637543), Key(16737080395141468053), Key(13977415469856941557), Key(1052117838564742180), Key(9424311196719389450), Key(12167498318705983564), Key(4301764225574437137), Key(17360266336634281276), Key(13868884065264943813), Key(15952283905104982306), Key(4998386290424363477), Key(4893239286087369377), Key(17573528852960048629), Key(2412201799238683587), Key(16517545668683925387), Key(16978748896271686395), Key(8830712609912112615), Key(244676446090624528), Key(10801320743593590304), Key(13531918303924845431), Key(10527125009130628070), Key(17495106538955645767), Key(14203433425689676251), Key(13760149572603586785), Key(1273129856199637694), Key(3154213753511759364), Key(12760143787594064657), Key(1600035040276021173), Key(5414819345072334853), Key(7201040945210650872), Key(11015789609492649674), Key(7712150959425383900), Key(8543311100722720016), Key(13076185511676908731), Key(3922562784470822468), Key(2780562387024492132), Key(6697120216501611455), Key(13480343126040452106), Key(12173667680050468927), Key(3302171945877565923), Key(16568602182162993491), Key(14953223006496535120), Key(16457941142416543492), ],
        [
            Key(2945262940327718556), Key(3775538624233802005), Key(4292201895252289600), Key(16433809973923446677), Key(1284774014851141252), Key(18314932087213148495), Key(8946796353798605717), Key(16445820069092145103), Key(7588664147775519679), Key(12896594212779880816), Key(14935880823937687725), Key(13400879436137989525), Key(13846969535995712591), Key(12484917729738156524), Key(17882592831712409952), Key(16637473249645425632), Key(15098223454147433904), Key(17631249017957605294), Key(12582001597670293135), Key(17902661106057732664), Key(10274060743048400565), Key(12005958760542442625), Key(6324932172735347303), Key(17192330553585486663), Key(9422872207407330841), Key(3177237980255163711), Key(14998024116488875998), Key(705793604453777656), Key(11327568552142987041), Key(7029368612848231507), Key(11062860980165499825), Key(2900628512702115887), Key(308431256844078091), Key(752802454931337639), Key(5576583881995601144), Key(8733594096989903760), Key(290737499942622970), Key(8992780576699235245), Key(10425616809589311900), Key(5493674620779310265), Key(12589103349525344891), Key(14857852059215963628), Key(13495551423272463104), Key(6944056268429507318), Key(3988842613368812515), Key(14815775969275954512), Key(17868612272134391879), Key(8436706119115607049), Key(7555807622404432493), Key(9144495607954586305), Key(6794801016890317083), Key(6072558259768997948), Key(10941535447546794938), Key(14043502401785556544), Key(8362621443508695308), Key(17736840905212253027), Key(2733031211210449030), Key(4350365705834634871), Key(1100550212031776323), Key(17430963890314521917), Key(7470064030368587841), Key(13387014036020469860), Key(7078824284187344392), Key(12312007608706932222), ],
        [
            Key(3826719064958106391), Key(17580452432494632735), Key(4372818848456885156), Key(20778095608392735), Key(9517712183106565981), Key(16772576131911258204), Key(12158847832281029501), Key(18318866654963083744), Key(14355784966049388499), Key(1442237715923966096), Key(16767620159370203923), Key(13501017873225644439), Key(12414460951753850741), Key(1630390626826320339), Key(11056926288496765292), Key(17514919132679636196), Key(6737125905271376420), Key(3156370395448333753), Key(7372374977020439436), Key(5277883516136612451), Key(16544956564115640970), Key(14431129579433994133), Key(10776067565185448), Key(15235680854177679657), Key(12767627681826077225), Key(1324675096273909386), Key(3456463189867507715), Key(9195964142578403484), Key(10627443539470127577), Key(7083655917886846512), Key(14734414825071094346), Key(8833975264052769557), Key(2965232458494052289), Key(12786367183060552144), Key(6364751811635930008), Key(12304694438192434386), Key(4420057912710567321), Key(13121826629733594974), Key(3295424378969736960), Key(16543444358261923928), Key(13665696745413941685), Key(3585618043384929225), Key(14758422515963078108), Key(5444185746065710993), Key(6217807121864929894), Key(7617121805124236390), Key(2176332518208481987), Key(1435617355844826626), Key(17897291909516933347), Key(17430612766366810879), Key(13845907184570465897), Key(3432307431600566936), Key(2532253559171451888), Key(11643128737472459646), Key(13606171979107604790), Key(10012509558550373270), Key(5587706015190365982), Key(18189230678861289336), Key(5637318834313874969), Key(4728172345191419793), Key(13287099661014164329), Key(8475766932330124954), Key(2781312650135424674), Key(10552294945874175633), ],
        [
            Key(14116194119706301666), Key(908994258594572803), Key(3835251526534030662), Key(3902806174142003247), Key(8404113168045990162), Key(10605456791970677788), Key(8371724936653327204), Key(10149265301602815302), Key(10280163375965480302), Key(12878458563073396434), Key(1480273033205949154), Key(15420639285122262859), Key(16040433549230388361), Key(10889445127567090568), Key(7154846977618541400), Key(15324267473561911299), Key(9123044315927273855), Key(18178395620988860923), Key(13937825686985326355), Key(6208640256728026680), Key(17803354189602776349), Key(8168466387959732965), Key(4747388335999020093), Key(8076893647775627477), Key(135355862477779318), Key(13727020784074293322), Key(16471001867829363208), Key(3944848361583366045), Key(6153835027004876065), Key(17541053953916494135), Key(830442639195732299), Key(5707759661195251524), Key(16745928189385382169), Key(13853872449862111272), Key(10763276423780512808), Key(528748578239178413), Key(1195366693239264477), Key(16072813688416096526), Key(9411878730995839744), Key(14250860229846220116), Key(3391112600086567492), Key(11283764167692931512), Key(1672248607577385754), Key(2130286739811077583), Key(18311727561747759139), Key(974583822133342724), Key(5061116103402273638), Key(3126855720952116346), Key(578870949780164607), Key(3776778176701636327), Key(14213795876687685078), Key(5613780124034108946), Key(6069741268072432820), Key(8893641350514130178), Key(15249957078178864452), Key(18092583129505773527), Key(11393903435307203091), Key(8119660695860781220), Key(13766130452052543028), Key(7096579372531132405), Key(7459026647266724422), Key(5897616920394564481), Key(4162427946331299898), Key(2527789185948800525), ],
        [
            Key(17290988795360054066), Key(5240905960030703813), Key(12532957579127022568), Key(7321214839249116978), Key(17188130528816882357), Key(13649660060729335176), Key(7877670809777050873), Key(8603165736220767331), Key(3731409983944574110), Key(14311591814980160037), Key(16719365103710912831), Key(15645061390881301878), Key(15313601992567477463), Key(558437165307320475), Key(10107592147679710958), Key(217058993405149273), Key(11583857652496458642), Key(12813267508475749642), Key(12801463184548517903), Key(10205205656182355892), Key(12009517757124415757), Key(11711220569788417590), Key(601506575385147719), Key(2403800598476663693), Key(3185273191806365666), Key(16311384682203900813), Key(2147738008043402447), Key(11784653004849107439), Key(11363702615030984814), Key(4459820841160151625), Key(17238855191434604666), Key(16533107622905015899), Key(12580437090734268666), Key(9002238121826321187), Key(7209727037264965188), Key(15210303941751662984), Key(5957580827072516578), Key(16077971979351817631), Key(7451935491114626499), Key(14243752318712699139), Key(12737894796843349185), Key(1351996896321498360), Key(4395539424431256646), Key(14636926406778905296), Key(10637364485216545239), Key(4709900282812548306), Key(14703591130731831913), Key(1476367765688281237), Key(4113914727206496161), Key(8066049843497142643), Key(7809561412546830570), Key(4879538739185105394), Key(9498083046807871856), Key(17559505952950827343), Key(11763387757765891631), Key(10055035698587107604), Key(12844734664424373030), Key(330991544207939447), Key(8508732305896661743), Key(11153570973223855023), Key(10238055872248257461), Key(1773280948989896239), Key(8300833427522849187), Key(10832779467616436194), ],
        [Key(0); 64],
        [Key(0); 64],
        [
            Key(11781789245711860189), Key(2747882707407274161), Key(3724767368808293169), Key(10298180063630105197), Key(10746438658164496957), Key(16037040440297371558), Key(17588125462232966688), Key(6880843334474042246), Key(560415017990002212), Key(6626394159937994533), Key(2670333323665803600), Key(4280458366389177326), Key(1467978672011198404), Key(7620133404071416883), Key(13350367343504972530), Key(10138430730509076413), Key(6785953884329063615), Key(4006903721835701728), Key(17529175408771439641), Key(2257868868401674686), Key(16350586259217027048), Key(12792669610269240489), Key(15445432911128260212), Key(3830919760132254685), Key(17463139367032047470), Key(15002266175994648649), Key(17680514289072042202), Key(362761448860517629), Key(2620716836644167551), Key(10876826577342073644), Key(14704635783604247913), Key(8370308497378149181), Key(16902199073103511157), Key(4712050710770633961), Key(2335277171236964126), Key(15454330651988402294), Key(6039398895644425870), Key(5330935207425949713), Key(6844204079868621004), Key(15018633515897982115), Key(5869887878873962697), Key(9619421978703093664), Key(7065039212033014872), Key(14085021312833583897), Key(17738639966636660046), Key(18274309123980813514), Key(16007640215959475868), Key(4326793000252505639), Key(11694193434453531305), Key(15789397716808962025), Key(8672273831614123897), Key(6109915657282875177), Key(6240221177136276484), Key(17650760467278016265), Key(13635783915766085055), Key(17178975703249397658), Key(690100752037560272), Key(846594232046156050), Key(11437611220054444781), Key(1050411833588837386), Key(10485589741397417446), Key(12844414679888429939), Key(6491358656106542835), Key(12575464921310399912), ],
        [
            Key(14923825269739949453), Key(18375002115249413557), Key(3423036550911737589), Key(15250861506191355802), Key(15031961129285356212), Key(15435012606837965840), Key(6304673951675292305), Key(12785716655315370815), Key(9808873325341612945), Key(9783992785966697331), Key(18138650430907468530), Key(18431297401347671031), Key(18148129570815566817), Key(12696743950740820713), Key(1854845205476015706), Key(12865777516920439176), Key(15636159047245426328), Key(17373407353156678628), Key(2495834645782650553), Key(11247757644603045972), Key(17130748698210142189), Key(11422966446976074719), Key(1595016003613213710), Key(3899856913033553150), Key(15470414105568996654), Key(2572459120480840982), Key(14288318049370965601), Key(4034656711994978492), Key(3619462250265206907), Key(12564616267900212223), Key(6563888989859451823), Key(2454157599688795602), Key(122761158351497116), Key(4118064480546384385), Key(13825342760651713002), Key(3757958894065091138), Key(3348351562535718824), Key(11085064257829065607), Key(4791949565677098244), Key(16741859899153424134), Key(13552228277894027114), Key(18043793947072687525), Key(18232133385309552782), Key(17162542170033385071), Key(17966719644677930276), Key(4126374944389900134), Key(7694029693525104626), Key(7844796758498075948), Key(15171322352384637386), Key(4901284706517591019), Key(11550611493505829690), Key(8591758722916550176), Key(6614280899913466481), Key(15659292666557594854), Key(8334845918197067198), Key(14303347218899317731), Key(18185681713739197231), Key(10010957749676186008), Key(6151588837035247399), Key(15955998980864570780), Key(14725804664707294906), Key(9071111217904025772), Key(4268551186589045976), Key(3787505694838293655), ],
        [
            Key(3463765996898474975), Key(1419043948633899671), Key(4738255775972431200), Key(10880687006345860054), Key(6083956890523873398), Key(15399367780949709721), Key(10077652868536637496), Key(4763774200646997281), Key(2058719554631509711), Key(16245257579300202929), Key(12549234361408101229), Key(5132111825598353706), Key(13210867931726967807), Key(8049587883156206974), Key(14208790774466773366), Key(15004789243215417478), Key(2705161721287640173), Key(6606951690346399114), Key(9038858141657157738), Key(9864507686211087503), Key(8174211780307618304), Key(16060351410629081351), Key(5484951598904056885), Key(12456759525904287919), Key(8919252620379965524), Key(15501107657356591656), Key(3242949188225361282), Key(5926058172544675863), Key(6405123151097452666), Key(172567736958909523), Key(17292315564005737229), Key(13464278685013338817), Key(3686053955562449182), Key(8857017014241158725), Key(15421895718306499875), Key(3815913251318905694), Key(3432648465599995302), Key(818320788389300537), Key(4071520112108071604), Key(13295466432639272442), Key(2426572569594491679), Key(10076303268977391406), Key(8784192232334006419), Key(2997181738853009670), Key(15770398685934330580), Key(13017264784195056557), Key(4330776497582490757), Key(10934498588458332802), Key(10356579632341837397), Key(2098241031318749487), Key(14789448409803449028), Key(11251433970760721438), Key(7224004101031043677), Key(15038935143876354117), Key(13215483265469582733), Key(1462298635979286935), Key(5759284467508932139), Key(5761810302276021825), Key(1946852319481058342), Key(8779292626819401953), Key(9980275774854520963), Key(9018156077605645253), Key(10175632970326281074), Key(17670251009423356428), ],
        [
            Key(2047473063754745880), Key(4129462703004022451), Key(10030514736718131075), Key(8457187454173219884), Key(675824455430313366), Key(15722708499135010396), Key(1416150021210949828), Key(18340753630988628266), Key(4279562020148953383), Key(7599717795808621650), Key(8493385059263161629), Key(5448373608430482181), Key(7975000343659144004), Key(3661443877569162353), Key(17436434418308603210), Key(7723061412912586436), Key(12478269109366344372), Key(5260527761162561230), Key(3664808336308943032), Key(12246522629121956498), Key(11421384233946319246), Key(10711232448204740396), Key(394033332107778027), Key(1653867462011650260), Key(10614247855083729040), Key(3511207051989217747), Key(14828688729293007936), Key(12730238737606105501), Key(9131161340116597330), Key(10475424158865388660), Key(12216784836515690585), Key(12605719261947498045), Key(55059904350528673), Key(5668017292185949458), Key(5318848626170854652), Key(5812165408168894719), Key(12436591089168384586), Key(11456184110470635333), Key(17354703890556504985), Key(12819708191444916183), Key(2051969874001439467), Key(9752086654524583546), Key(8598830537031500033), Key(10803717843971298140), Key(17386254373003795027), Key(3490013643061567317), Key(14966160920336416174), Key(2716159408585464742), Key(13704057180721116715), Key(6139827121406310950), Key(12045645008689575811), Key(5879666907986225363), Key(18332108852121545326), Key(8302596541641486393), Key(3337300269606353125), Key(4641043901128821440), Key(17552658021160699704), Key(15245517114959849830), Key(898774234328201642), Key(13458365488972458856), Key(17617352963801145870), Key(12653043169047643133), Key(3946055118622982785), Key(78667567517654999), ],
        [
            Key(7496345100749090134), Key(11141138397664383499), Key(9990861652354760086), Key(6136051413974204120), Key(14382251659553821084), Key(12222838175704680581), Key(9437743647758681312), Key(5321952072316248116), Key(9510472571572253025), Key(13968738580144591953), Key(9048732621241245672), Key(7070992119077796289), Key(7585987196905721881), Key(12797609451470009512), Key(13831169997283951441), Key(14062956797276305407), Key(7195172102806297836), Key(13763135782447679404), Key(8729177333120200902), Key(8228513033455726756), Key(5827889096510108059), Key(1541817158620711182), Key(18002525473269359251), Key(7210349805272776282), Key(6760744891923215431), Key(1684012349959865632), Key(5422658641223860702), Key(5964630753289401637), Key(16048931659747747714), Key(12995369105282084360), Key(2210225853011473806), Key(13310794355402477849), Key(4356361331354780175), Key(10920940233470324174), Key(4480682637160025854), Key(11920920861864075275), Key(17830720560385394644), Key(17667812763781863653), Key(8584251371203620679), Key(10083927648945854194), Key(15175717840117055506), Key(3402388332801799152), Key(17983756367024412696), Key(13633521765968038314), Key(18197623828188242686), Key(7159151014196207335), Key(6329323109608928752), Key(4596348075478973761), Key(1929043772203993371), Key(2942782730029388844), Key(17616535832761962408), Key(14638746212880920282), Key(235408037287298392), Key(15488773953079788133), Key(14511691540381881087), Key(4908241668947178463), Key(8002325218109467205), Key(384694259305835297), Key(4413022859932656147), Key(16084510603130945976), Key(7817184652260023923), Key(11521163704900182019), Key(10633473972031941012), Key(7028123206539359005), ],
        [
            Key(12370129909167185711), Key(18282545875249343957), Key(11571910781648655955), Key(12044362528788437371), Key(15748959137105604538), Key(12433669315838447795), Key(3539341563356477798), Key(8229636981602574987), Key(18267920850505015981), Key(18135187956959905864), Key(10122403804874825725), Key(8577640427585662579), Key(16947872026033056961), Key(4498886674923994328), Key(5110446196942225801), Key(2443501881669395127), Key(6915148508579620831), Key(9154422921438056207), Key(3578030806440286511), Key(15315801991440539300), Key(7070866824836391168), Key(14817924832942381111), Key(3001446271118775643), Key(13000642695841600636), Key(14370567463871457833), Key(11030064684553339453), Key(14239970918075645415), Key(9415971121016597759), Key(6665243610733579451), Key(12729882327349519727), Key(127495542892799647), Key(6044073010763988256), Key(13007064564721953048), Key(13888665226332397302), Key(13536486134713258398), Key(16493663995181111698), Key(2130152061385863810), Key(5369940202574713097), Key(4976109024626592507), Key(17662718886951473514), Key(10194604604769366768), Key(9434649875492567077), Key(9275344374679790988), Key(13950395516943844512), Key(4634019286100624619), Key(17524913661501655732), Key(12758868016771465513), Key(3127147764315865797), Key(3960938717909563730), Key(14869830638616427590), Key(305185646789997459), Key(4139658351799906696), Key(272667046354598132), Key(15621274402096728762), Key(16483498129229512495), Key(12953368655171389128), Key(10678035399177741929), Key(18049652274331575310), Key(7975081034372805163), Key(10522098076497821829), Key(12606359703294662790), Key(13924857104548874958), Key(6566773282407180921), Key(3452471826952569846), ],
        [Key(0); 64],
    ];
    const ENPASSANT: [Key; 8] = [
        Key(9031641776876329352), Key(12228382040141709029), Key(2494223668561036951), Key(7849557628814744642), Key(16000570245257669890), Key(16614404541835922253), Key(17787301719840479309), Key(6371708097697762807), ]
    ;
    const CASTLING: [Key; 16] = [
        Key(0), Key(7487338029351702425), Key(10138645747811604478), Key(16959407016388712551), Key(16332212992845378228), Key(9606164174486469933), Key(7931993123235079498), Key(719529192282958547), Key(6795873897769436611), Key(4154453049008294490), Key(15203167020455580221), Key(13048090984296504740), Key(13612242447579281271), Key(15780674830245624046), Key(3484610688987504777), Key(6319549394931232528), ];
    const SIDE: Key = Key(4906379431808431525);
    const NO_PAWNS: Key = Key(895963052000028445);

    pub fn psq(pc: Piece, s: Square) -> Key {
        PSQ[pc.0 as usize][s.0 as usize]
    }

    pub fn material(pc: Piece, num: i32) -> Key {
        PSQ[pc.0 as usize][num as usize]
    }

    pub fn enpassant(f: File) -> Key {
        ENPASSANT[f as usize]
    }

    pub fn castling(cr: CastlingRight) -> Key {
        CASTLING[cr.0 as usize]
    }

    pub fn side() -> Key {
        SIDE
    }

    pub fn no_pawns() -> Key {
        NO_PAWNS
    }
}

#[derive(Clone)]
pub struct StateInfo {
    // Copied when making a move
    pub pawn_key: Key,
    pub material_key: Key,
    pub non_pawn_material: [Value; 2],
    pub castling_rights: CastlingRight,
    pub rule50: i32,
    pub plies_from_null: i32,
    pub psq: Score,
    pub ep_square: Square,

    // Not copied when making a move (will be recomputed anyhow)
    pub key: Key,
    pub checkers_bb: Bitboard,
    pub captured_piece: Piece,
    pub blockers_for_king: [Bitboard; 2],
    pub pinners_for_king: [Bitboard; 2],
    pub check_squares: [Bitboard; 8],
}

impl StateInfo {
    pub fn new() -> StateInfo {
        StateInfo {
            pawn_key: Key(0),
            material_key: Key(0),
            non_pawn_material: [Value::ZERO; 2],
            castling_rights: CastlingRight(0),
            rule50: 0,
            plies_from_null: 0,
            psq: Score::ZERO,
            ep_square: Square::NONE,
            key: Key(0),
            checkers_bb: Bitboard(0),
            captured_piece: NO_PIECE,
            blockers_for_king: [Bitboard(0); 2],
            pinners_for_king: [Bitboard(0); 2],
            check_squares: [Bitboard(0); 8],
        }
    }

    pub fn copy(&self) -> StateInfo {
        StateInfo {
            pawn_key: self.pawn_key,
            material_key: self.material_key,
            non_pawn_material: self.non_pawn_material,
            castling_rights: self.castling_rights,
            rule50: self.rule50,
            plies_from_null: self.plies_from_null,
            psq: self.psq,
            ep_square: self.ep_square,
            key: Key(0),
            checkers_bb: Bitboard(0),
            captured_piece: NO_PIECE,
            blockers_for_king: [Bitboard(0); 2],
            pinners_for_king: [Bitboard(0); 2],
            check_squares: [Bitboard(0); 8],
        }
    }
}

pub struct Position {
    board: [Piece; 64],
    by_color_bb: [Bitboard; 2],
    by_type_bb: [Bitboard; 8],
    piece_count: [i32; 16],
    piece_list: [[Square; 16]; 16],
    index: [i32; 64],
    castling_rights_mask: [CastlingRight; 64],
    castling_rook_square: [Square; 16],
    castling_path: [Bitboard; 16],
    game_ply: i32,
    side_to_move: Color,
    states: Vec<StateInfo>,
    chess960: bool,
    // Thread variables from here
    // only for main thread:
    pub failed_low: bool,
    pub best_move_changes: f64,
    pub previous_time_reduction: f64,
    pub previous_score: Value,
    pub calls_cnt: i32,
    // for all threads:
    pub thread_ctrl: Option<Arc<ThreadCtrl>>,
    pub is_main: bool,
    pub thread_idx: i32,
    pub pv_idx: usize,
    pub pv_last: usize,
    pub sel_depth: i32,
    pub nmp_ply: i32,
    pub nmp_odd: i32,
    pub nodes: u64,
    pub tb_hits: u64,
    pub completed_depth: Depth,
    pub root_moves: search::RootMoves,
    // thread-specific tables
    pub pawns_table: Vec<std::cell::UnsafeCell<pawns::Entry>>,
    pub material_table: Vec<std::cell::UnsafeCell<material::Entry>>,
    pub counter_moves: CounterMoveHistory,
    pub main_history: ButterflyHistory,
    pub capture_history: CapturePieceToHistory,
    pub cont_history: ContinuationHistory,
}

impl Position {
    pub fn new() -> Position {
        Position {
            board: [NO_PIECE; 64],
            by_color_bb: [Bitboard(0); 2],
            by_type_bb: [Bitboard(0); 8],
            piece_count: [0; 16],
            piece_list: [[Square::NONE; 16]; 16],
            index: [0; 64],
            castling_rights_mask: [CastlingRight(0); 64],
            castling_rook_square: [Square::NONE; 16],
            castling_path: [Bitboard(0); 16],
            game_ply: 0,
            side_to_move: WHITE,
            states: Vec::new(),
            chess960: false,
            failed_low: false,
            best_move_changes: 0.0,
            previous_time_reduction: 0.0,
            previous_score: Value::ZERO,
            calls_cnt: 0,
            thread_ctrl: None,
            is_main: false,
            thread_idx: 0,
            pv_idx: 0,
            pv_last: 0,
            sel_depth: 0,
            nmp_ply: 0,
            nmp_odd: 0,
            nodes: 0,
            tb_hits: 0,
            completed_depth: Depth::ZERO,
            root_moves: Vec::new(),
            pawns_table: Vec::new(),
            material_table: Vec::new(),
            counter_moves: unsafe { std::mem::zeroed() },
            main_history: unsafe { std::mem::zeroed() },
            capture_history: unsafe { std::mem::zeroed() },
            cont_history: unsafe { std::mem::zeroed() },
        }
    }

    pub fn init_states(&mut self) {
        self.states.truncate(0);
        self.states.push(StateInfo::new());
    }

    fn st(&self) -> &StateInfo {
        self.states.last().unwrap()
    }

    fn st_mut(&mut self) -> &mut StateInfo {
        self.states.last_mut().unwrap()
    }

    pub fn side_to_move(&self) -> Color {
        self.side_to_move
    }

    pub fn empty(&self, s: Square) -> bool {
        self.board[s.0 as usize] == NO_PIECE
    }

    pub fn piece_on(&self, s: Square) -> Piece {
        self.board[s.0 as usize]
    }

    pub fn moved_piece(&self, m: Move) -> Piece {
        self.board[m.from().0 as usize]
    }

    pub fn pieces(&self) -> Bitboard {
        self.by_type_bb[ALL_PIECES.0 as usize]
    }

    pub fn pieces_p(&self, pt: PieceType) -> Bitboard {
        self.by_type_bb[pt.0 as usize]
    }

    pub fn pieces_pp(&self, pt1: PieceType, pt2: PieceType) -> Bitboard {
        self.pieces_p(pt1) | self.pieces_p(pt2)
    }

    pub fn pieces_c(&self, c: Color) -> Bitboard {
        self.by_color_bb[c.0 as usize]
    }

    pub fn pieces_cp(&self, c: Color, pt: PieceType) -> Bitboard {
        self.pieces_c(c) & self.pieces_p(pt)
    }

    pub fn pieces_cpp(
        &self, c: Color, pt1: PieceType, pt2: PieceType,
    ) -> Bitboard {
        self.pieces_c(c) & self.pieces_pp(pt1, pt2)
    }

    pub fn count(&self, c: Color, pt: PieceType) -> i32 {
        self.piece_count[Piece::make(c, pt).0 as usize]
    }

    pub fn squares(&self, c: Color, pt: PieceType) -> &[Square] {
        &self.piece_list[Piece::make(c, pt).0 as usize]
    }

    pub fn square_list(&self, c: Color, pt: PieceType) -> SquareList {
        SquareList::construct(self.squares(c, pt))
    }

    pub fn square(&self, c: Color, pt: PieceType) -> Square {
        self.squares(c, pt)[0]
    }

    pub fn ep_square(&self) -> Square {
        self.st().ep_square
    }

    pub fn has_castling_right(&self, cr: CastlingRight) -> bool {
        self.st().castling_rights & cr != 0
    }

    pub fn castling_rights(&self, c: Color) -> CastlingRight {
        self.st().castling_rights & CastlingRight(3 << (2 * c.0))
    }

    pub fn can_castle(&self, c: Color) -> bool {
        self.castling_rights(c) != 0
    }

    pub fn castling_impeded(&self, cr: CastlingRight) -> bool {
        self.pieces() & self.castling_path[cr.0 as usize] != Bitboard(0)
    }

    pub fn castling_rook_square(&self, cr: CastlingRight) -> Square {
        self.castling_rook_square[cr.0 as usize]
    }

    pub fn attacks_from_pawn(&self, s: Square, c: Color) -> Bitboard {
        pawn_attacks(c, s)
    }

    pub fn attacks_from(&self, pt: PieceType, s: Square) -> Bitboard {
        debug_assert!(pt != PAWN);
        match pt {
            BISHOP | ROOK => attacks_bb(pt, s, self.pieces()),
            QUEEN => self.attacks_from(ROOK, s) | self.attacks_from(BISHOP, s),
            _ => pseudo_attacks(pt, s)
        }
    }

    pub fn attackers_to_occ(&self, s: Square, occ: Bitboard) -> Bitboard {
        (self.attacks_from_pawn(s, BLACK) & self.pieces_cp(WHITE, PAWN))
            | (self.attacks_from_pawn(s, WHITE) & self.pieces_cp(BLACK, PAWN))
            | (self.attacks_from(KNIGHT, s) & self.pieces_p(KNIGHT))
            | (attacks_bb(ROOK, s, occ) & self.pieces_pp(ROOK, QUEEN))
            | (attacks_bb(BISHOP, s, occ) & self.pieces_pp(BISHOP, QUEEN))
            | (self.attacks_from(KING, s) & self.pieces_p(KING))
    }

    pub fn attackers_to(&self, s: Square) -> Bitboard {
        self.attackers_to_occ(s, self.by_type_bb[ALL_PIECES.0 as usize])
    }

    pub fn checkers(&self) -> Bitboard {
        self.st().checkers_bb
    }

    pub fn blockers_for_king(&self, c: Color) -> Bitboard {
        self.st().blockers_for_king[c.0 as usize]
    }

    pub fn pinners_for_king(&self, c: Color) -> Bitboard {
        self.st().pinners_for_king[c.0 as usize]
    }

    pub fn check_squares(&self, pt: PieceType) -> Bitboard {
        self.st().check_squares[pt.0 as usize]
    }

    pub fn pawn_passed(&self, c: Color, s: Square) -> bool {
        self.pieces_cp(!c, PAWN) & passed_pawn_mask(c, s) == 0
    }

    pub fn advanced_pawn_push(&self, m: Move) -> bool {
        self.moved_piece(m).piece_type() == PAWN
            && m.from().relative_rank(self.side_to_move()) > RANK_4
    }

    pub fn key(&self) -> Key {
        self.st().key
    }

    pub fn pawn_key(&self) -> Key {
        self.st().pawn_key
    }

    pub fn material_key(&self) -> Key {
        self.st().material_key
    }

    pub fn psq_score(&self) -> Score {
        self.st().psq
    }

    pub fn non_pawn_material_c(&self, c: Color) -> Value {
        self.st().non_pawn_material[c.0 as usize]
    }

    pub fn non_pawn_material(&self) -> Value {
        self.non_pawn_material_c(WHITE) + self.non_pawn_material_c(BLACK)
    }

    pub fn game_ply(&self) -> i32 {
        self.game_ply
    }

    pub fn rule50_count(&self) -> i32 {
        self.st().rule50
    }

    pub fn opposite_bishops(&self) -> bool {
        self.piece_count[W_BISHOP.0 as usize] == 1
            && self.piece_count[B_BISHOP.0 as usize] == 1
            && opposite_colors(self.square(WHITE, BISHOP),
                               self.square(BLACK, BISHOP))
    }

    pub fn is_chess960(&self) -> bool {
        self.chess960
    }

    pub fn capture_or_promotion(&self, m: Move) -> bool {
        debug_assert!(m.is_ok());
        if m.move_type() != NORMAL {
            m.move_type() != CASTLING
        } else {
            !self.empty(m.to())
        }
    }

    pub fn capture(&self, m: Move) -> bool {
        debug_assert!(m.is_ok());
        (!self.empty(m.to()) && m.move_type() != CASTLING)
            || m.move_type() == ENPASSANT
    }

    pub fn captured_piece(&self) -> Piece {
        self.st().captured_piece
    }

    pub const PIECE_TO_CHAR: &'static str = " PNBRQK  pnbrqk";

    pub fn print(&mut self) {
        println!("\n +---+---+---+---+---+---+---+---+");
        for r in (0..8).rev() {
            for f in 0..8 {
                print!(" | {}", Position::PIECE_TO_CHAR.chars()
                    .nth(self.piece_on(Square::make(f, r)).0 as usize)
                    .unwrap());
            }
            println!(" |\n +---+---+---+---+---+---+---+---+");
        }

        println!("\nFen: {}\nKey: {}\nCheckers: {}", self.fen(), self.key(),
                 self.checkers());

        if tb::max_cardinality() >= Bitboard::pop_count(self.pieces())
            && !self.has_castling_right(ANY_CASTLING)
        {
            let mut s1 = 1;
            let mut s2 = 1;
            let wdl = tb::probe_wdl(self, &mut s1);
            let dtz = tb::probe_dtz(self, &mut s2);
            println!("Tablebases WDL: {} ({})\nTablebases DTZ: {} ({})",
                     wdl, s1, dtz, s2);
            if s1 != 0 {
                let dtm = tb::probe_dtm(self, wdl, &mut s1);
                println!("Tablebases DTM: {} ({})", uci::value(dtm), s1);
            }
        }
    }

    // set() initializes the position objection with the given FEN string.
    // This function is not very robust - make sure that input FENs are
    // correct. This is assumed to be the responsibility of the GUI.

    pub fn set(&mut self, fen_str: &str, is_chess960: bool) {
        for c in 0..2 {
            self.by_color_bb[c] = Bitboard(0);
        }
        for t in 0..8 {
            self.by_type_bb[t] = Bitboard(0);
        }
        for i in 0..16 {
            self.piece_count[i] = 0;
            self.castling_path[i] = Bitboard(0);
            self.castling_rook_square[i] = Square::NONE;
            for j in 0..16 {
                self.piece_list[i][j] = Square::NONE;
            }
        }
        for i in 0..64 {
            self.board[i] = NO_PIECE;
            self.castling_rights_mask[i] = CastlingRight(0);
        }

        let mut iter = fen_str.split_whitespace();

        // 1. Piece placement
        let pieces = iter.next().unwrap();
        let mut sq = Square::A8;
        for c in pieces.chars() {
            if let Some(d) = c.to_digit(10) {
                sq += (d as i32) * EAST; // Advance the given number of files
            } else if c == '/' {
                sq += 2 * SOUTH;
            } else if let Some(idx) = Position::PIECE_TO_CHAR.find(c) {
                self.put_piece(Piece(idx as u32), sq);
                sq += EAST;
            }
        }

        // 2. Active color
        let color = iter.next().unwrap();
        self.side_to_move = if color == "b" { BLACK } else { WHITE };

        // 3. Castling availability. Compatible with 3 standards: Normal FEN
        // standard, Shredder-FEN that uses the letters of the columns on
        // which the rooks began the game instead of KQkq and also X-FEN
        // standard that, in case of Chess960, if an inner rook is associated
        // with the castling right, the castling tag is replaced by the file
        // letter of the involved rook, as for the Shredder-FEN.
        let castling = iter.next().unwrap();
        if castling != "-" {
            for c in castling.chars() {
                let color = if c.is_lowercase() { BLACK } else { WHITE };
                let rook = Piece::make(color, ROOK);
                let side = c.to_uppercase().next().unwrap();
                let mut rsq;
                if side == 'K' {
                    rsq = Square::H1.relative(color);
                    while self.piece_on(rsq) != rook {
                        rsq += WEST;
                    }
                } else if side == 'Q' {
                    rsq = Square::A1.relative(color);
                    while self.piece_on(rsq) != rook {
                        rsq += EAST;
                    }
                } else if ('A'..='H').contains(&side) {
                    let file = side.to_digit(18).unwrap() - 10;
                    rsq = Square::make(file, relative_rank(color, RANK_1));
                } else {
                    continue;
                }
                self.set_castling_right(color, rsq);
            }
        }

        // 4. En passant square. Ignore if no pawn capture is possible
        let enpassant = iter.next().unwrap();
        self.st_mut().ep_square = Square::NONE;
        if enpassant != "-" {
            let file = enpassant.chars().next().unwrap();
            let file = file.to_digit(18).unwrap() - 10;
            let rank = if self.side_to_move == WHITE { 5 } else { 2 };
            let ep_sq = Square::make(file, rank);
            if self.attackers_to(ep_sq)
                & self.pieces_cp(self.side_to_move, PAWN) != 0
                && self.pieces_cp(!self.side_to_move, PAWN)
                & (ep_sq + pawn_push(!self.side_to_move)) != 0
            {
                self.st_mut().ep_square = ep_sq;
            }
        }

        // 5-6. Halfmove clock and fullmove number
        if let Some(halfmove) = iter.next() {
            self.st_mut().rule50 = halfmove.parse().unwrap();
        } else {
            self.st_mut().rule50 = 0;
        }

        // Convert from fullmove starting from 1 to game_ply starting from 0.
        // Handle also common incorrect FEN with fullmove = 0.
        if let Some(fullmove) = iter.next() {
            let fullmove = fullmove.parse::<i32>().unwrap();
            self.game_ply = std::cmp::max(2 * (fullmove - 1), 0);
        } else {
            self.game_ply = 0;
        }
        if self.side_to_move == BLACK {
            self.game_ply += 1;
        }

        self.chess960 = is_chess960;
        self.set_state();

        debug_assert!(self.is_ok());
    }

    // set_castling_right() is a helper function used to set castling rights
    // given the corresponding color and the rook starting square.

    fn set_castling_right(&mut self, c: Color, rfrom: Square) {
        let kfrom = self.square(c, KING);
        let cs = if kfrom < rfrom { CastlingSide::KING } else { CastlingSide::QUEEN };
        let cr = c | cs;

        self.st_mut().castling_rights |= cr;
        self.castling_rights_mask[kfrom.0 as usize] |= cr;
        self.castling_rights_mask[rfrom.0 as usize] |= cr;
        self.castling_rook_square[cr.0 as usize] = rfrom;

        let kto = relative_square(c,
                                  if cs == CastlingSide::KING { Square::G1 } else { Square::C1 });
        let rto = relative_square(c,
                                  if cs == CastlingSide::KING { Square::F1 } else { Square::D1 });

        let mut s = std::cmp::min(rfrom, rto);
        while s <= std::cmp::max(rfrom, rto) {
            if s != kfrom && s != rfrom {
                self.castling_path[cr.0 as usize] |= s;
            }
            s += EAST;
        }

        let mut s = std::cmp::min(kfrom, kto);
        while s <= std::cmp::max(kfrom, kto) {
            if s != kfrom && s != rfrom {
                self.castling_path[cr.0 as usize] |= s;
            }
            s += EAST;
        }
    }

    // set_check_info() sets king attacks to detect if a move gives cehck

    fn set_check_info(&mut self) {
        let mut pinners = Bitboard(0);
        self.st_mut().blockers_for_king[WHITE.0 as usize] =
            self.slider_blockers(self.pieces_c(BLACK),
                                 self.square(WHITE, KING), &mut pinners);
        self.st_mut().pinners_for_king[WHITE.0 as usize] = pinners;
        self.st_mut().blockers_for_king[BLACK.0 as usize] =
            self.slider_blockers(self.pieces_c(WHITE),
                                 self.square(BLACK, KING), &mut pinners);
        self.st_mut().pinners_for_king[BLACK.0 as usize] = pinners;

        let ksq = self.square(!self.side_to_move(), KING);

        self.st_mut().check_squares[PAWN.0 as usize] =
            self.attacks_from_pawn(ksq, !self.side_to_move);
        self.st_mut().check_squares[KNIGHT.0 as usize] =
            self.attacks_from(KNIGHT, ksq);
        self.st_mut().check_squares[BISHOP.0 as usize] =
            self.attacks_from(BISHOP, ksq);
        self.st_mut().check_squares[ROOK.0 as usize] =
            self.attacks_from(ROOK, ksq);
        self.st_mut().check_squares[QUEEN.0 as usize] =
            self.st().check_squares[BISHOP.0 as usize]
                | self.st().check_squares[ROOK.0 as usize];
        self.st_mut().check_squares[KING.0 as usize] = Bitboard(0);
    }

    // set_state() computes the hash keys of the position, and other data
    // that once computed is updated incrementally as moves are made.
    // The function is used only when a new position is set up, and to verify
    // the correctness of the StateInfo data when running in debug mode.

    fn set_state(&mut self) {
        self.st_mut().key = Key(0);
        self.st_mut().material_key = Key(0);
        self.st_mut().pawn_key = zobrist::no_pawns();
        self.st_mut().non_pawn_material[WHITE.0 as usize] = Value::ZERO;
        self.st_mut().non_pawn_material[BLACK.0 as usize] = Value::ZERO;
        self.st_mut().psq = Score::ZERO;
        self.st_mut().checkers_bb =
            self.attackers_to(self.square(self.side_to_move, KING))
                & self.pieces_c(!self.side_to_move);

        self.set_check_info();

        for s in self.pieces() {
            let pc = self.piece_on(s);
            self.st_mut().key ^= zobrist::psq(pc, s);
            self.st_mut().psq += psqt::psq(pc, s);
        }

        if self.st_mut().ep_square != Square::NONE {
            let tmp = zobrist::enpassant(self.st().ep_square.file());
            self.st_mut().key = tmp;
        }

        if self.side_to_move == BLACK {
            self.st_mut().key ^= zobrist::side();
        }

        {
            let tmp = zobrist::castling(self.st().castling_rights);
            self.st_mut().key ^= tmp;
        }

        for s in self.pieces_p(PAWN) {
            let tmp = zobrist::psq(self.piece_on(s), s);
            self.st_mut().pawn_key ^= tmp;
        }

        for c in 0..2 {
            for pt in 2..6 {
                let pc = Piece::make(Color(c), PieceType(pt));
                let tmp =
                    self.count(Color(c), PieceType(pt)) * piece_value(MG, pc);
                self.st_mut().non_pawn_material[c as usize] += tmp;
            }

            for pt in 1..7 {
                let pc = Piece::make(Color(c), PieceType(pt));
                for cnt in 0..self.count(Color(c), PieceType(pt)) {
                    self.st_mut().material_key ^= zobrist::material(pc, cnt);
                }
            }
        }
    }

    // fen() returns a FEN representation of the position. In case of Chess960
    // the Shredder-FEN notation is used.

    pub fn fen(&self) -> String {
        let mut ss = String::new();

        for r in (0..8).rev() {
            let mut f = 0;
            while f < 8 {
                let mut empty_cnt = 0u8;
                while f < 8 && self.empty(Square::make(f, r)) {
                    empty_cnt += 1;
                    f += 1;
                }
                if empty_cnt > 0 {
                    ss.push((48u8 + empty_cnt) as char);
                }
                if f < 8 {
                    let c = Position::PIECE_TO_CHAR.chars()
                        .nth(self.piece_on(Square::make(f, r)).0 as usize)
                        .unwrap();
                    ss.push(c);
                    f += 1;
                }
            }
            if r > 0 {
                ss.push('/');
            }
        }

        ss.push_str(if self.side_to_move == WHITE { " w " } else { " b " });

        self.castle_helper(&mut ss, WHITE_OO, 'K');
        self.castle_helper(&mut ss, WHITE_OOO, 'Q');
        self.castle_helper(&mut ss, BLACK_OO, 'k');
        self.castle_helper(&mut ss, BLACK_OOO, 'q');

        if !self.has_castling_right(ANY_CASTLING) {
            ss.push('-');
        }

        if self.ep_square() == Square::NONE {
            ss.push_str(" - ");
        } else {
            ss.push(' ');
            ss.push_str(&uci::square(self.ep_square()));
            ss.push(' ');
        }

        ss.push_str(&self.rule50_count().to_string());
        ss.push(' ');
        ss.push_str(&(1 + self.game_ply() / 2).to_string());

        ss
    }

    fn castle_helper(&self, ss: &mut String, cr: CastlingRight, c: char) {
        if !self.has_castling_right(cr) {
            return;
        }

        if !self.chess960 {
            ss.push(c);
        } else {
            let f = self.castling_rook_square(cr).file();
            let r = self.castling_rook_square(cr).rank();
            let mut c = 65 + f;
            if r == RANK_8 {
                c += 32;
            }
            ss.push((c as u8) as char);
        }
    }

    // slider_blockers() returns a bitboard of all the pieces (both colors)
    // that are blocking attacks on the square 's' from 'sliders'. A piece
    // blocks a slider if removing that piece from the board would result
    // in a position where square 's'is attacked. For example, a king attack
    // blocking piece can be either a pinned or a discovered check piece,
    // depending on whether its color is the opposite of or the same as the
    // color of the slider.

    pub fn slider_blockers(
        &self, sliders: Bitboard, s: Square, pinners: &mut Bitboard,
    ) -> Bitboard {
        let mut blockers = Bitboard(0);
        *pinners = Bitboard(0);

        // Snipers are sliders that attack 's' when a piece is removed
        let snipers =
            ((pseudo_attacks(ROOK, s) & self.pieces_pp(QUEEN, ROOK))
                | (pseudo_attacks(BISHOP, s) & self.pieces_pp(QUEEN, BISHOP)))
                & sliders;

        for sniper_sq in snipers {
            let b = between_bb(s, sniper_sq) & self.pieces();

            if !more_than_one(b) {
                blockers |= b;
                if b & self.pieces_c(self.piece_on(s).color()) != 0 {
                    *pinners |= sniper_sq;
                }
            }
        }
        blockers
    }

    // legal() tests whether a pseudo-legal move is legal

    pub fn legal(&self, m: Move) -> bool {
        debug_assert!(m.is_ok());

        let us = self.side_to_move;
        let from = m.from();

        debug_assert!(self.moved_piece(m).color() == us);
        debug_assert!(
            self.piece_on(self.square(us, KING)) == Piece::make(us, KING)
        );

        // En passant captures are a tricky special case. Because they are
        // uncommon, we do it simply by testing whether the king is attacked
        // after the move is made.
        if m.move_type() == ENPASSANT {
            let ksq = self.square(us, KING);
            let to = m.to();
            let capsq = to - pawn_push(us);
            let occupied = (self.pieces() ^ from ^ capsq) | to;

            debug_assert!(to == self.ep_square());
            debug_assert!(self.moved_piece(m) == Piece::make(us, PAWN));
            debug_assert!(self.piece_on(capsq) == Piece::make(!us, PAWN));
            debug_assert!(self.piece_on(to) == NO_PIECE);

            return
                attacks_bb(ROOK, ksq, occupied)
                    & self.pieces_cpp(!us, QUEEN, ROOK) == 0
                    && attacks_bb(BISHOP, ksq, occupied)
                    & self.pieces_cpp(!us, QUEEN, BISHOP) == 0;
        }

        // If the moving piece is a king, check whether the destination
        // square is attacked by the opponent. Castling moves are checked
        // for legality during move generation.
        if self.piece_on(from).piece_type() == KING {
            return m.move_type() == CASTLING
                || self.attackers_to(m.to()) & self.pieces_c(!us) == 0;
        }

        // A non-king move is legal if and only if it is not pinned or it
        // is moving along the ray towards or away from the king.
        self.blockers_for_king(us) & from == 0
            || aligned(from, m.to(), self.square(us, KING))
    }

    // pseudo_legal() takes a random move and tests whether the move is
    // pseudo legal. It is used to validate moves from T that can be
    // corrupted due to SMP concurrent access or hash position key aliasing.

    pub fn pseudo_legal(&self, m: Move) -> bool {
        let us = self.side_to_move();
        let from = m.from();
        let to = m.to();
        let pc = self.moved_piece(m);

        // Use a slower but simpler function for uncommon cases
        if m.move_type() != NORMAL {
            return MoveList::new::<Legal>(self).contains(m);
        }

        // It is not a promotion, so promotion piece must be empty
        if m.promotion_type() != KNIGHT {
            return false;
        }

        // If the 'from' square is not occupied by a piece belonging to the
        // side to move, the move is obviously not legal.
        if pc == NO_PIECE || pc.color() != us {
            return false;
        }

        // The destination square cannot be occupied by a friendly piece
        if self.pieces_c(us) & to != 0 {
            return false;
        }

        // Handle the special case of a pawn move
        if pc.piece_type() == PAWN {
            // We have already handled promotion moves, so destination
            // cannot be on the 8th/1st rank.
            if to.rank() == relative_rank(us, RANK_8) {
                return false;
            }

            if self.attacks_from_pawn(from, us) & self.pieces_c(!us) & to == 0
                && !((from + pawn_push(us) == to) && self.empty(to))
                && !(from + 2 * pawn_push(us) == to
                && from.rank() == relative_rank(us, RANK_2)
                && self.empty(to)
                && self.empty(to - pawn_push(us)))
            {
                return false;
            }
        } else if self.attacks_from(pc.piece_type(), from) & to == 0 {
            return false;
        }

        // Evasions generator already takes care of avoiding certain kinds of
        // illegal moves and legal() relies on this. We therefore have to take
        // care that the same kind of moves are filtered out here.
        if self.checkers() != 0 {
            if pc.piece_type() != KING {
                // Double check? In this case a king move is required
                if more_than_one(self.checkers()) {
                    return false;
                }

                // Our move must be a blocking evasion or a capture of the
                // checking piece
                if (between_bb(lsb(self.checkers()), self.square(us, KING))
                    | self.checkers()) & to == 0
                {
                    return false;
                }
            }
            // In case of king moves under check we have to remove king so as
            // to catch invalid moves like b1a1 when opposite queen is on c1.
            else if self.attackers_to_occ(to, self.pieces() ^ from)
                & self.pieces_c(!us) != 0
            {
                return false;
            }
        }

        true
    }

    // gives_check() tests whether a pseudo-legal move gives a check

    pub fn gives_check(&self, m: Move) -> bool {
        debug_assert!(m.is_ok());
        debug_assert!(self.moved_piece(m).color() == self.side_to_move());

        let from = m.from();
        let to = m.to();

        // Is there a direct check?
        if self.st().check_squares[self.piece_on(from).piece_type().0 as usize]
            & to != 0
        {
            return true;
        }

        // Is there a discovered check?
        if self.blockers_for_king(!self.side_to_move()) & from != 0
            && !aligned(from, to, self.square(!self.side_to_move(), KING))
        {
            return true;
        }

        match m.move_type() {
            NORMAL => false,

            PROMOTION => {
                attacks_bb(m.promotion_type(), to, self.pieces() ^ from)
                    & self.square(!self.side_to_move(), KING) != 0
            }

            // En passant capture with check? We have already handled the
            // case of direct checks and ordinary discovered check, so the
            // only case we need to handle is the unusual case of a
            // discovered check through the captured pawn.
            ENPASSANT => {
                let capsq = Square::make(to.file(), from.rank());
                let b = (self.pieces() ^ from ^ capsq) | to;

                (attacks_bb(ROOK, self.square(!self.side_to_move(), KING), b)
                    & self.pieces_cpp(self.side_to_move(), QUEEN, ROOK))
                    | (attacks_bb(BISHOP,
                                  self.square(!self.side_to_move(), KING),
                                  b)
                    & self.pieces_cpp(self.side_to_move(), QUEEN, BISHOP)) != 0
            }

            CASTLING => {
                let kfrom = from;
                let rfrom = to; // Castling is encoded as king captures rook
                let kto = relative_square(self.side_to_move(),
                                          if rfrom > kfrom { Square::G1 } else { Square::C1 });
                let rto = relative_square(self.side_to_move(),
                                          if rfrom > kfrom { Square::F1 } else { Square::D1 });

                (pseudo_attacks(ROOK, rto)
                    & self.square(!self.side_to_move(), KING)) != 0
                    && (attacks_bb(ROOK, rto,
                                   (self.pieces() ^ kfrom ^ rfrom) | rto | kto)
                    & self.square(!self.side_to_move(), KING)) != 0
            }

            _ => {
                debug_assert!(false);
                false
            }
        }
    }

    // do_move() makes a move and saves all information necessary to a
    // StateInfo object. The move is assumed to be legal. Pseudo-legal
    // moves should be filtered out before this function is called.

    pub fn do_move(&mut self, m: Move, gives_check: bool) {
        debug_assert!(m.is_ok());

        self.nodes += 1;
        let mut k = self.st().key ^ zobrist::side();

        // Copy some fields of the old state to our new StateInfo object
        // except the ones which are going to be recalculated from scratch
        // anyway.
        let st_copy = self.st().copy();
        self.states.push(st_copy);

        // Increment ply counters. The rule50 field will be reset to zero
        // later on in case of a capture or a pawn move.
        self.game_ply += 1;
        self.st_mut().rule50 += 1;
        self.st_mut().plies_from_null += 1;

        let us = self.side_to_move();
        let them = !us;
        let from = m.from();
        let mut to = m.to();
        let pc = self.piece_on(from);
        let mut captured =
            if m.move_type() == ENPASSANT {
                Piece::make(them, PAWN)
            } else {
                self.piece_on(to)
            };

        debug_assert!(pc.color() == us);
        debug_assert!(captured == NO_PIECE
            || captured.color() ==
            if m.move_type() != CASTLING { them } else { us }
        );

        if m.move_type() == CASTLING {
            debug_assert!(pc == Piece::make(us, KING));
            debug_assert!(captured == Piece::make(us, ROOK));

            let mut rfrom = Square::A1;
            let mut rto = Square::A1;
            self.do_castling::<True>(us, from, &mut to, &mut rfrom, &mut rto);

            self.st_mut().psq +=
                psqt::psq(captured, rto) - psqt::psq(captured, rfrom);
            k ^= zobrist::psq(captured, rfrom) ^ zobrist::psq(captured, rto);
            captured = NO_PIECE;
        }

        if captured != NO_PIECE {
            let mut capsq = to;

            // If the captured piece is a pawn, update pawn hash key, otherwise
            // update non-pawn material.
            if captured.piece_type() == PAWN {
                if m.move_type() == ENPASSANT {
                    capsq -= pawn_push(us);

                    debug_assert!(pc == Piece::make(us, PAWN));
                    debug_assert!(to == self.st_mut().ep_square);
                    debug_assert!(to.relative_rank(us) == RANK_6);
                    debug_assert!(self.piece_on(to) == NO_PIECE);
                    debug_assert!(
                        self.piece_on(capsq) == Piece::make(them, PAWN)
                    );

                    self.board[capsq.0 as usize] = NO_PIECE;
                }

                self.st_mut().pawn_key ^= zobrist::psq(captured, capsq);
            } else {
                self.st_mut().non_pawn_material[them.0 as usize] -=
                    piece_value(MG, captured);
            }

            // Update board and piece lists
            self.remove_piece(captured, capsq);

            // Update material hash key and prefetch access to material_table
            k ^= zobrist::psq(captured, capsq);
            {
                let tmp = zobrist::material(captured,
                                            self.piece_count[captured.0 as usize]);
                self.st_mut().material_key ^= tmp;
            }
            // prefetch

            // Update incremental scores
            self.st_mut().psq -= psqt::psq(captured, capsq);

            // Reset rule 50 counter
            self.st_mut().rule50 = 0;
        }

        // Update hash key
        k ^= zobrist::psq(pc, from) ^ zobrist::psq(pc, to);

        // Reset en passant square
        if self.st_mut().ep_square != Square::NONE {
            k ^= zobrist::enpassant(self.st().ep_square.file());
            self.st_mut().ep_square = Square::NONE;
        }

        // Update castling rights if needed
        if self.st_mut().castling_rights != 0
            && self.castling_rights_mask[from.0 as usize]
            | self.castling_rights_mask[to.0 as usize] != 0
        {
            let cr = self.castling_rights_mask[from.0 as usize]
                | self.castling_rights_mask[to.0 as usize];
            k ^= zobrist::castling(self.st().castling_rights & cr);
            self.st_mut().castling_rights &= !cr;
        }

        // Move the piece. The tricky Chess960 castling is handled earlier
        if m.move_type() != CASTLING {
            self.move_piece(pc, from, to);
        }

        // If the moving piece is a pawn do some special extra work
        if pc.piece_type() == PAWN {
            // Set en-passant square if the moved pawn can be captured
            if to.0 ^ from.0 == 16
                && self.attacks_from_pawn(to - pawn_push(us), us)
                & self.pieces_cp(them, PAWN) != 0
            {
                self.st_mut().ep_square = to - pawn_push(us);
                k ^= zobrist::enpassant(self.st().ep_square.file());
            } else if m.move_type() == PROMOTION {
                let promotion = Piece::make(us, m.promotion_type());

                debug_assert!(to.relative_rank(us) == RANK_8);
                debug_assert!(promotion.piece_type() >= KNIGHT
                    && promotion.piece_type() <= QUEEN);

                self.remove_piece(pc, to);
                self.put_piece(promotion, to);

                // Update hash keys
                k ^= zobrist::psq(pc, to) ^ zobrist::psq(promotion, to);
                self.st_mut().pawn_key ^= zobrist::psq(pc, to);
                {
                    let tmp = zobrist::material(promotion,
                                                self.piece_count[promotion.0 as usize] - 1)
                        ^ zobrist::material(pc,
                                            self.piece_count[pc.0 as usize]);
                    self.st_mut().material_key ^= tmp;
                }

                // Update incremental score
                self.st_mut().psq +=
                    psqt::psq(promotion, to) - psqt::psq(pc, to);

                // Update material
                self.st_mut().non_pawn_material[us.0 as usize] +=
                    piece_value(MG, promotion);
            }

            // Update pawn hash key and prefetch access to pawns_table
            self.st_mut().pawn_key ^=
                zobrist::psq(pc, from) ^ zobrist::psq(pc, to);
            // prefetch2(...);

            // Reset rule 50 draw counter
            self.st_mut().rule50 = 0;
        }

        // Update incremental scores
        self.st_mut().psq += psqt::psq(pc, to) - psqt::psq(pc, from);

        // Set captured piece
        self.st_mut().captured_piece = captured;

        // Update the key with the final value
        self.st_mut().key = k;

        // Calculate checkers bitboard (if move gives check)
        self.st_mut().checkers_bb =
            if gives_check {
                self.attackers_to(self.square(them, KING)) & self.pieces_c(us)
            } else {
                Bitboard(0)
            };

        self.side_to_move = them;

        // Update king attacks used for fast check detection
        self.set_check_info();

        debug_assert!(self.is_ok());
    }

    // undo_move() unmakes a move. When it returns, the position should be
    // restored to exactly the same state as before the move was made.

    pub fn undo_move(&mut self, m: Move) {
        debug_assert!(m.is_ok());

        self.side_to_move = !self.side_to_move;

        let us = self.side_to_move;
        let from = m.from();
        let mut to = m.to();
        let mut pc = self.piece_on(to);

        debug_assert!(self.empty(from) || m.move_type() == CASTLING);
        debug_assert!(self.st().captured_piece.piece_type() != KING);

        if m.move_type() == PROMOTION {
            debug_assert!(to.relative_rank(us) == RANK_8);
            debug_assert!(pc.piece_type() == m.promotion_type());
            debug_assert!(
                pc.piece_type() >= KNIGHT && pc.piece_type() <= QUEEN);

            self.remove_piece(pc, to);
            pc = Piece::make(us, PAWN);
            self.put_piece(pc, to);
        }

        if m.move_type() == CASTLING {
            let mut rfrom = Square(0);
            let mut rto = Square(0);
            self.do_castling::<False>(us, from, &mut to, &mut rfrom, &mut rto);
        } else {
            // Put the piece back at the source square
            self.move_piece(pc, to, from);

            if self.st().captured_piece != NO_PIECE {
                let mut capsq = to;

                if m.move_type() == ENPASSANT {
                    capsq -= pawn_push(us);

                    debug_assert!(pc.piece_type() == PAWN);
                    debug_assert!(to.relative_rank(us) == RANK_6);
                    debug_assert!(self.piece_on(capsq) == NO_PIECE);
                    debug_assert!(
                        self.st().captured_piece == Piece::make(!us, PAWN));
                }

                // Restore the captured piece
                let cap_piece = self.st().captured_piece;
                self.put_piece(cap_piece, capsq);
            }
        }

        let new_len = self.states.len() - 1;
        self.states.truncate(new_len);
        self.game_ply -= 1;

        debug_assert!(self.is_ok());
    }

    // do_castling() is a helper used to do/undo a castling move. This is
    // a bit tricky in Chess960 where from/to squares can overlap.
    fn do_castling<Do: Bool>(
        &mut self, us: Color, from: Square, to: &mut Square,
        rfrom: &mut Square, rto: &mut Square,
    ) {
        let king_side = *to > from;
        *rfrom = *to; // Castling is encoded as king captures rook
        *rto = relative_square(us,
                               if king_side { Square::F1 } else { Square::D1 });
        *to = relative_square(us,
                              if king_side { Square::G1 } else { Square::C1 });

        // Remove both pieces first since squares could overlap in Chess960
        self.remove_piece(Piece::make(us, KING),
                          if Do::BOOL { from } else { *to });
        self.remove_piece(Piece::make(us, ROOK),
                          if Do::BOOL { *rfrom } else { *rto });
        self.board[(if Do::BOOL { from } else { *to }).0 as usize] = NO_PIECE;
        self.board[(if Do::BOOL { *rfrom } else { *rto }).0 as usize] =
            NO_PIECE;
        self.put_piece(Piece::make(us, KING),
                       if Do::BOOL { *to } else { from });
        self.put_piece(Piece::make(us, ROOK),
                       if Do::BOOL { *rto } else { *rfrom });
    }

    // do(undo)_null_move() is used to do(undo) a "null move": it flips the
    // side to move without executing any move on the board.

    pub fn do_null_move(&mut self) {
        debug_assert!(self.checkers() == 0);

        let st_copy = (*self.st()).clone(); // full copy
        self.states.push(st_copy);

        if self.st().ep_square != Square::NONE {
            let tmp = zobrist::enpassant(self.st().ep_square.file());
            self.st_mut().key ^= tmp;
            self.st_mut().ep_square = Square::NONE;
        }

        self.st_mut().key ^= zobrist::side();
        // prefetch

        self.st_mut().rule50 += 1;
        self.st_mut().plies_from_null = 0;

        self.side_to_move = !self.side_to_move;

        self.set_check_info();

        debug_assert!(self.is_ok());
    }

    pub fn undo_null_move(&mut self) {
        debug_assert!(self.checkers() == 0);

        let new_len = self.states.len() - 1;
        self.states.truncate(new_len);
        self.side_to_move = !self.side_to_move;
    }

    // key_after() computes the new hash key after the given move. Needed
    // for specualtive prefetch. It does not recognize special moves like
    // castling, en-passant and promotions.

    #[allow(dead_code)]
    fn key_after(&self, m: Move) -> Key {
        let from = m.from();
        let to = m.to();
        let pc = self.piece_on(from);
        let captured = self.piece_on(to);
        let mut k = self.st().key ^ zobrist::side();

        if captured != NO_PIECE {
            k ^= zobrist::psq(captured, to);
        }

        k ^ zobrist::psq(pc, to) ^ zobrist::psq(pc, from)
    }

    // see_ge() tests if the SEE value of move is greater than or equal to
    // the given threshold. We use an algorithm similar to alpha-beta pruning
    // with a null window.

    pub fn see_ge(&self, m: Move, value: Value) -> bool {
        debug_assert!(m.is_ok());

        // Only deal with normal moves, assume others pass a simple see
        if m.move_type() != NORMAL {
            return Value::ZERO >= value;
        }

        let from = m.from();
        let to = m.to();

        // The opponent may be able to recapture so this is the best result
        // we can hope for.
        let mut swap = piece_value(MG, self.piece_on(to)) - value;
        if swap < Value::ZERO {
            return false;
        }

        // Now assume the worst possible result: that the opponent can
        // capture our piece for free.
        swap = piece_value(MG, self.piece_on(from)) - swap;
        if swap <= Value::ZERO {
            return true;
        }

        // Find all attackers to the destination square, with the moving piece
        // removed, but possibly an X-ray attacked added behind it.
        let mut occ = self.pieces() ^ from ^ to;
        let mut stm = self.piece_on(from).color();
        let mut attackers = self.attackers_to_occ(to, occ);
        let mut res = Value(1);

        loop {
            stm = !stm;
            attackers &= occ;
            let mut stm_attackers = attackers & self.pieces_c(stm);
            if stm_attackers == 0 {
                break;
            }
            if stm_attackers & self.blockers_for_king(stm) != 0
                && self.pinners_for_king(stm) & !occ == 0
            {
                stm_attackers &= !self.blockers_for_king(stm);
            }
            if stm_attackers == 0 {
                break;
            }
            res = Value(res.0 ^ 1);
            let bb = stm_attackers & self.pieces_p(PAWN);
            if bb != 0 {
                swap = PawnValueMg - swap;
                if swap < res {
                    break;
                }
                occ ^= bb & -bb;
                attackers |=
                    attacks_bb(BISHOP, to, occ) & self.pieces_pp(BISHOP, QUEEN);
                continue;
            }
            let bb = stm_attackers & self.pieces_p(KNIGHT);
            if bb != 0 {
                swap = KnightValueMg - swap;
                if swap < res {
                    break;
                }
                occ ^= bb & -bb;
                continue;
            }
            let bb = stm_attackers & self.pieces_p(BISHOP);
            if bb != 0 {
                swap = BishopValueMg - swap;
                if swap < res {
                    break;
                }
                occ ^= bb & -bb;
                attackers |=
                    attacks_bb(BISHOP, to, occ) & self.pieces_pp(BISHOP, QUEEN);
                continue;
            }
            let bb = stm_attackers & self.pieces_p(ROOK);
            if bb != 0 {
                swap = RookValueMg - swap;
                if swap < res {
                    break;
                }
                occ ^= bb & -bb;
                attackers |=
                    attacks_bb(ROOK, to, occ) & self.pieces_pp(ROOK, QUEEN);
                continue;
            }
            let bb = stm_attackers & self.pieces_p(QUEEN);
            if bb != 0 {
                swap = QueenValueMg - swap;
                if swap < res {
                    break;
                }
                occ ^= bb & -bb;
                attackers |=
                    (attacks_bb(BISHOP, to, occ)
                        & self.pieces_pp(BISHOP, QUEEN))
                        | (attacks_bb(ROOK, to, occ) & self.pieces_pp(ROOK, QUEEN));
                continue;
            }
            return if attackers & !self.pieces_c(stm) != 0 {
                res == Value::ZERO
            } else {
                res != Value::ZERO
            };
        }
        res != Value::ZERO
    }

    // is_draw() tests whether the position is drawn by 50-move rule or by
    // repetition. It does not detect stalemates.

    pub fn is_draw(&self, ply: i32) -> bool {
        if self.st().rule50 > 99
            && (self.checkers() == 0
            || MoveList::new::<Legal>(self).len() != 0)
        {
            return true;
        }

        let end = std::cmp::min(self.st().rule50, self.st().plies_from_null);

        if end < 4 {
            return false;
        }

        let mut k = self.states.len() - 3;
        let mut cnt = 0;

        let mut i = 4;
        while i <= end {
            k -= 2;

            // Return a draw score if a position repeats once earlier but
            // strictly after the root, or repeats twice before or at the
            // root.
            if self.states[k].key == self.st().key {
                cnt += 1;
                if cnt + ((ply > i) as i32) == 2 {
                    return true;
                }
            }

            i += 2;
        }

        false
    }

    pub fn has_repeated(&self) -> bool {
        let mut l = self.states.len() - 1;
        loop {
            let mut i = 4;
            let e = std::cmp::min(self.states[l].rule50,
                                  self.states[l].plies_from_null);

            if e < i {
                return false;
            }

            let mut k = self.states.len() - 3;

            while i <= e {
                k -= 2;

                if self.states[k].key == self.states[l].key {
                    return true;
                }

                i += 2;
            }

            l -= 2;
        }
    }

    fn put_piece(&mut self, pc: Piece, s: Square) {
        self.board[s.0 as usize] = pc;
        self.by_type_bb[ALL_PIECES.0 as usize] |= s;
        self.by_type_bb[pc.piece_type().0 as usize] |= s;
        self.by_color_bb[pc.color().0 as usize] |= s;
        self.index[s.0 as usize] = self.piece_count[pc.0 as usize];
        self.piece_count[pc.0 as usize] += 1;
        self.piece_list[pc.0 as usize][self.index[s.0 as usize] as usize] = s;
        self.piece_count[Piece::make(pc.color(), ALL_PIECES).0 as usize] += 1;
    }

    fn remove_piece(&mut self, pc: Piece, s: Square) {
        self.by_type_bb[ALL_PIECES.0 as usize] ^= s;
        self.by_type_bb[pc.piece_type().0 as usize] ^= s;
        self.by_color_bb[pc.color().0 as usize] ^= s;
        self.piece_count[pc.0 as usize] -= 1;
        let last_square = self.piece_list[pc.0 as usize]
            [self.piece_count[pc.0 as usize] as usize];
        self.index[last_square.0 as usize] = self.index[s.0 as usize];
        self.piece_list[pc.0 as usize]
            [self.index[last_square.0 as usize] as usize] = last_square;
        self.piece_list[pc.0 as usize]
            [self.piece_count[pc.0 as usize] as usize] = Square::NONE;
        self.piece_count[Piece::make(pc.color(), ALL_PIECES).0 as usize] -= 1;
    }

    fn move_piece(&mut self, pc: Piece, from: Square, to: Square) {
        let from_to_bb = from.bb() ^ to.bb();
        self.by_type_bb[ALL_PIECES.0 as usize] ^= from_to_bb;
        self.by_type_bb[pc.piece_type().0 as usize] ^= from_to_bb;
        self.by_color_bb[pc.color().0 as usize] ^= from_to_bb;
        self.board[from.0 as usize] = NO_PIECE;
        self.board[to.0 as usize] = pc;
        self.index[to.0 as usize] = self.index[from.0 as usize];
        self.piece_list[pc.0 as usize][self.index[to.0 as usize] as usize] =
            to;
    }

    // is_ok() performs some consistency checks for the position object and
    // raises an assert if something wrong is detected. This is meant to be
    // helpful when debugging.

    pub fn is_ok(&self) -> bool {
        if self.side_to_move() != WHITE && self.side_to_move != BLACK
            || self.piece_on(self.square(WHITE, KING)) != W_KING
            || self.piece_on(self.square(BLACK, KING)) != B_KING
            || (self.ep_square() != Square::NONE
            && self.ep_square().relative_rank(self.side_to_move())
            != RANK_6)
        {
            panic!("pos: Default");
        }

        if self.count(WHITE, KING) != 1
            || self.count(BLACK, KING) != 1
            || self.attackers_to(self.square(!self.side_to_move(), KING))
            & self.pieces_c(self.side_to_move()) != 0
        {
            panic!("pos_is_ok: Kings");
        }

        if self.pieces_p(PAWN) & (RANK1_BB | RANK8_BB) != 0
            || self.count(WHITE, PAWN) > 8
            || self.count(BLACK, PAWN) > 8
        {
            panic!("pos_is_ok: Pawns");
        }

        for p1 in 1..6 {
            for p2 in 1..6 {
                if p1 != p2
                    && self.pieces_p(PieceType(p1))
                    & self.pieces_p(PieceType(p2)) != 0
                {
                    panic!("pos_is_ok: Bitboards");
                }
            }
        }

        for p in 1..14 {
            if p == 7 || p == 8 {
                continue;
            }
            let pc = Piece(p);
            if self.piece_count[pc.0 as usize] !=
                Bitboard::pop_count(self.pieces_cp(pc.color(), pc.piece_type())) as i32
            {
                panic!("pos_is_ok: Pieces {}", p);
            }

            for i in 0..self.piece_count[pc.0 as usize] {
                if self.board
                    [self.piece_list[pc.0 as usize][i as usize].0 as usize]
                    != pc
                    || self.index
                    [self.piece_list[pc.0 as usize][i as usize].0 as usize]
                    != i
                {
                    panic!("pos_is_ok: Index {}, {}", p, i);
                }
            }
        }

        true
    }
}
