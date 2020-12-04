use regex::Regex;


fn main() {
    part1and2();
}

fn part1and2() {
    let re = Regex::new(r"(?m)^(\d{1,2})-(\d{1,2}) ([a-z]): ([a-z]+)$").unwrap();
    let mut good_1 = 0;
    let mut good_2 = 0;
    for cap in re.captures_iter(INPUT) {
        let min = *&cap[1].parse::<usize>().unwrap();
        let max = *&cap[2].parse::<usize>().unwrap();
        let key = &cap[3];
        let psw = &cap[4];
        let matches = psw.matches(key).count();
        if matches >= min && matches <= max {
            good_1 += 1;
        }
        let key_char = key.chars().next().unwrap();
        let chars = &psw.chars();
        let first = *&psw.chars().nth(min - 1).unwrap_or('\0');
        let second = *&psw.chars().nth(max - 1).unwrap_or('\0');
        let in_first = first == key_char;
        let in_second = second == key_char;
        println!("{}, Key: {}, at {}: {}, at {}: {}", &cap[0], key_char, min, first, max, second);
        if (in_first || in_second) && !(in_first && in_second) {
            good_2 += 1;
        }
    }
    println!("Part 1: There are {} good passwords", good_1);
    println!("Part 2: There are {} good passwords", good_2);
}

const INPUT: &str = "\
2-5 z: zzztvz
2-8 d: pddzddkdvqgxndd
4-14 r: rrrjrrrrrrbrrccrr
2-7 r: zrgsnrr
9-10 z: zzzxwzznpd
8-13 g: gggggggxgggghggg
1-6 c: xcccxcccccz
3-4 b: bxbt
8-11 d: dddddddzddv
4-14 m: kxdmmmdmfwmmmdfr
16-17 g: ggggggggggggggggg
2-3 f: ddvdlff
9-11 g: ggggpfgggggg
7-8 c: fdhctccc
3-6 c: tmcdcncqcvccg
2-3 l: clllll
1-9 b: bbbbbbbbbb
10-15 w: wglmwwwrnnzgwhhwvvd
10-14 g: ggggggxsgggqggg
9-19 q: fjlqbvtdngwvtbnsgfm
1-2 k: kgbkkv
4-10 m: mmmmtmmmmgm
3-4 h: hplh
5-7 q: qqqqrqk
3-12 v: zbvlbpxcrnvvwjpwl
1-4 r: crrr
4-5 h: hhhrj
2-4 r: mhrr
9-10 c: pccccsccbk
10-16 n: drnhqnnnnqnnpxmh
5-6 v: vvvnvvg
11-12 c: ccccgcccccpc
5-10 b: bdbjbbfmfbb
9-13 w: zwbwwzhwwkwsxdfwglx
5-19 v: vvvvvqrvvfvgvvvvmvv
3-9 d: dddgztdljb
12-13 s: swssssrsssshx
5-6 l: dllkljq
4-5 q: qqqsdqq
2-4 w: tmwg
10-12 l: llllsldflllllljjlxl
3-11 s: sjtsssssksbs
1-5 v: hvvtmvgvv
3-12 g: ggvgkgfgggghgg
3-4 p: ccpp
3-9 v: vfvvvvvvvvvvvr
8-16 k: kkkkkkkkkkkkkwkkkk
8-9 x: wxbrxfdrw
7-9 n: nnnfnnnnn
7-11 w: wwwwxwwwwwzxw
6-9 n: nknnhnnnnlnnn
10-12 l: qllllllllqbl
17-19 p: pppppppppppppwppkpp
4-5 q: qqqqqq
7-15 v: vvvbmbjvjznzdhfvtn
3-12 w: wwwwwdwwwwwwtwwwww
7-10 x: mmgmknpvdbxdl
9-10 v: jljttxpvvv
11-15 d: dshddnddqdhddvm
1-3 r: vrqrrr
6-7 c: ltcbpccw
5-13 q: qqqqcqqqxqrqg
3-7 l: lhlmhzl
2-3 l: lxntjnttkllll
3-4 b: bbnsb
5-6 j: rjppzvjdjj
4-10 g: fgggjgvfggggggsggm
11-12 z: ztzzzkzzzzsvzzzzt
2-3 h: hcvbf
7-13 s: sswkwhsdqsdcs
3-10 j: mghjglnbdhjp
6-8 x: dfxrxxsxpsxxxxmc
11-12 f: fffmfcdpffmpflfffdw
4-7 n: nnnknncnn
2-6 k: tzqlzqhrncvktfwvswt
2-4 h: pqplh
8-13 h: zhlwqvmnkjhhtkf
2-3 l: llll
19-20 j: jjjjjjjjjjjjjjjjjrjj
3-14 n: crnnrgpmxxlrfljk
2-4 t: stpr
11-13 c: cctcccccccccccc
5-6 q: fqqqqqdwtwqjq
6-7 z: zzzzzzg
8-16 j: njjgsjjljktjcbjvjc
3-6 c: swjccc
5-7 h: ghvwjmzm
9-12 s: sssssmvtdckmfs
3-4 r: bsrm
6-7 c: ccctcccc
6-10 t: swndrtcvtl
13-19 v: vjvxvvvdvvvvvvdvblv
5-7 w: hnwwwtwp
8-9 k: kkkkkktzxkkv
4-6 g: xpsxgs
2-3 r: xrrdxm
2-9 f: ffvqxxvsf
7-10 t: tttttttttt
4-6 q: bbqqqqqqbq
7-9 q: xjcwqhprgh
3-6 j: jjjjjjjxjjj
1-3 l: blll
2-5 p: phbppp
3-4 p: phpb
2-15 r: mwgnrkxrrbmgpzrdrr
8-13 x: jsrlvxmxwglkh
4-11 b: ctcbsjgjplbthhrskfnb
6-7 q: qqqqqfvcq
10-11 d: ddddddjdbndddd
8-9 p: pppppppkf
10-14 q: qqqqqqnqzgqsqv
12-13 b: bbbbbbbbbbbbb
6-11 h: khnkthfhhdhrhh
5-6 g: gzggwcgq
2-4 f: fbfg
6-9 f: ffvfsfvffxff
6-7 t: tsfdltt
14-17 t: tttttttttttxtcttp
3-4 v: vwtv
7-10 x: pxmldqqxmw
1-2 s: pdssssssss
5-6 n: snwnml
3-8 t: tjnpjqzxbj
5-7 n: mxkqfnjb
4-12 q: qqqkqqqqqqqkqqq
4-5 d: pjhnsfdgddjsddvpxxd
14-15 t: trttdtxwtdmtttt
3-4 j: jjvz
1-2 p: xqpppp
11-15 d: dddpdddddddddddq
15-16 b: bnbbbbbbbbbtbbdbb
9-11 w: wwwnwwmwbghw
15-18 g: tjwqbgwmvpvghxgzkgj
3-4 j: jjps
9-12 f: ffflfffmfnffvff
1-3 g: gbgstndgggqrdbtpdqdr
4-6 f: fflxfqffff
10-14 t: ttttttttttttttttt
1-7 v: vvsqlvvvvkj
3-7 b: wvbxlwbjtbbbvh
3-7 p: hqvsvpn
2-6 f: fkwfhf
3-11 d: dzrwnffrdvs
8-9 s: sssdssztlssqksx
12-15 t: tttttttttthtttt
5-6 m: mmmmmmmm
2-3 n: dnnl
12-16 w: wrwwtvwwwwwwwwww
2-4 j: jjjjsj
7-11 x: xxxxxtcxxxlxxx
8-17 n: nbngnnnsnjnbhnntzwxt
5-6 b: tmbdgn
15-16 x: xqlpjxbbxnrbxxxxqmkf
1-3 c: ccclf
12-14 m: zhmmmzltwhrmfmx
15-17 c: ccvcxcmccgccccbcccc
10-13 k: mkgkljkkkjkkg
1-4 b: ckdr
10-12 b: bvfbgbbbbbbb
2-4 v: jfvcv
2-7 p: jswmqpmc
11-16 b: bbbncbpmbjbbvgrb
1-6 n: ktjcwr
2-9 l: lllrwljlnlwllln
17-18 h: hhhhhhhhhhhhhhhhzfh
9-14 b: bbbsbqbbjjbbkb
7-12 j: jjjjjwwjjjjcj
10-13 w: wfwwwwwwwjwww
2-6 j: bjghpj
13-16 z: zzzzzzzzbzzzzzvg
10-11 f: ffffkfffcfxff
3-4 m: mmmmkjvzmkp
2-5 k: kkkkk
5-6 r: rrbrrs
1-2 c: ccqjmc
13-14 r: rrrrrrrrrrrrztr
8-12 z: zrzzzzzqbzvdz
4-9 m: hlgmtvvqs
9-10 t: tthttrtbrnt
2-4 k: snpkzhck
12-13 x: wlxwwxxxhgxxx
11-16 h: ncmfhhvgghhhhhjhd
13-15 g: sgxgfntgjnxkgbgm
4-13 s: vqkslldmgmfrd
3-14 z: zzzzzzzzzzzzzzzz
14-15 c: ccccvccccccsmlpcfccx
4-5 p: pppppp
9-10 z: kgblzrqbzz
13-15 p: pppppptpmppbpppqpp
7-8 b: dntbbcmswb
16-17 d: dddddddddddddddgd
10-11 r: rrrsrmrrrqtr
2-3 q: qtqgz
6-8 c: xztzfkcch
14-15 l: lllflllllllllzsl
1-6 s: zssssss
5-8 d: hdddwddvdpd
10-12 j: jjjjjjjjjpjbj
1-9 q: tqqqqqqqqnqqwqk
2-4 t: shpwwttwqb
8-9 s: xrssssssz
3-4 c: cjcfc
3-10 v: rfvzsncmfv
4-7 t: lshczltjfqkmkt
13-16 x: xxxxxxxfxxxxzxxrxx
1-3 z: pzkzlzmzz
4-5 d: ddkbfdtdd
9-10 r: rrrrrrrrlfkj
3-7 s: svtbsssjsldts
10-13 c: ccccgxzqtctrc
1-7 q: zgwqgrx
6-8 f: ffnffrfg
4-5 f: kffcffp
1-2 m: ggcm
5-6 z: zzzdgz
9-14 j: jjjnjjjjjjjjjjjj
2-17 d: xttgzggnfhsrjqdbppps
5-6 f: xrcfff
1-3 z: wzzg
1-2 h: hhhvl
13-17 q: qqqqqqqqqqmqqqqqqq
2-3 q: bjqq
1-5 w: zwwwgww
5-7 s: jnvpmgj
9-10 q: zctgztwpqq
16-20 j: jkjjkjnjwqqjjjzgjjpk
16-20 k: kkkkkkkkkkkkkkkkkkkk
17-18 d: dddddddddddddddddbd
3-5 d: dngrhxb
10-12 t: tmttttttttrttwh
7-9 q: nqqqkqcqqqqq
7-9 z: zqzzzzwctng
10-13 d: dddjldsddkddjpddd
3-7 g: lgglgggggphx
6-7 b: llkbcdxwtzrblp
1-5 s: dssdscpzwcn
2-6 h: hhphhhhhh
4-13 z: smkzzpzjfzzbzqzxzjz
2-5 j: jjjjjj
8-11 p: lgmppspppppk
9-10 c: ccccccccdnkc
3-9 m: mmfmmmmpkmsmmm
13-15 m: mmmmkmmmmmmmgmhmmm
5-6 x: xxxxxxx
10-14 b: bbbjbqbbwcbbbb
3-12 f: ffmfffffffnzfffff
11-12 m: mmmnmmmmmmmmm
8-12 v: jptmbccvmvsxchfvv
4-5 s: ssssns
12-14 j: jjjjjjdjjjjvjhjjj
11-13 j: wjjjljjjcjjjjqjjp
1-7 g: ggggggggdg
2-3 v: bkjqv
4-13 q: qzhldwthbwgtr
12-13 p: ppplppdgpppppp
11-12 f: flffxfpffprgfff
8-14 b: mbbbbbtbwbhstbbbbbbb
6-7 m: bqmsmmm
1-3 w: qxlggmxtwj
3-17 c: dwbkjcwqcrccbcxcnvxc
11-13 p: ppxppppdppxpzpppp
4-10 v: vfvvfvzvqv
1-7 m: mmmxmmmmmbmmrmmmmmm
2-12 x: xrdxwxzwfxpxtqxxxgxp
6-11 w: swzwwkddmsrxwzfg
6-8 g: ggggggggggg
3-4 h: tzshs
7-8 t: zjtzpclm
5-6 k: brzkzk
14-15 c: cccccccccjccccc
2-3 h: hhfh
15-19 w: wwwwwwwkzxwwwwwwwwh
8-10 f: mjmwqnlnhf
3-8 l: ksllllplllc
1-3 q: gjmhq
2-9 l: lwklcxllf
5-7 p: ppcclscppgdxmg
3-6 z: fpxmpz
8-11 r: vrrrrrrrrrrrrrs
1-7 q: cqqqqqgq
13-14 t: tttztttttttttt
4-6 q: qzkqsqzmhhq
8-14 l: llllfllplllgzll
5-9 m: mmmtmmmmm
9-14 l: llllllllrllllltl
3-4 r: czrr
2-5 s: lssslssnmfqss
14-17 h: hhhhhhhhhhhhhhhhhh
3-4 x: xxxx
7-8 x: zdjbxxsl
10-13 n: ffrnphpzhnfxnncptmn
7-11 m: mmmmmmdmmmmmf
1-2 j: jjjjjjjjjtj
2-6 g: kglgsq
3-8 x: csvxxxxxztxdxpw
3-9 m: qbqwfmfpjmbnbcccgfgk
2-14 v: rmqhqlrvcjdhpw
8-9 p: pppppppvwpm
8-12 f: zvfjxbdgzjbplm
2-3 f: frff
7-9 f: htkxffclfr
4-7 v: cvqjctzvzvvv
6-7 f: fjffflj
1-4 b: tbnpb
8-10 t: tjpbtfzhtfsdjtlttttt
8-12 h: hhhhhhhhhhhrh
3-5 n: hxbwgjchklnplnb
1-2 z: jzvz
4-7 z: vzzzvcz
11-12 f: fffffvcfskcfcfftqf
2-5 p: cxktpckkqkgjpptvgp
9-16 b: bbbbbbbbkbbbbbbbbbbb
10-11 p: jcpnpppxpppb
7-9 n: sncnrzncnzdnngn
10-11 n: ttqrrkbspnn
11-19 c: ccczncccccsccpccccjc
1-2 h: hhhbwhhknzf
5-8 q: zxbtktkf
2-6 x: trxhzl
1-5 d: dskddkqdgqkhcz
7-12 c: ccccccccccccc
7-9 t: kskrrhtxtvsqzt
5-12 v: vvvjvwrdfscv
6-10 z: pzznznzczkwzzzmzz
15-18 t: tttttntttttttttttttt
4-6 m: bxmmmm
2-8 k: tkkhqwkkkpk
6-11 p: pppppgmpppppp
7-10 b: bkbcbbrrbr
6-11 c: cctccccbnbcvccc
2-5 t: ccpxgwzqktxwjf
4-11 s: sssssscsssssg
3-4 d: ndpk
3-4 z: gjzzzkbhzzs
8-9 t: ttttttttt
5-10 l: lllldmmlln
9-15 j: njzjgjjjjjjjprjjj
5-7 p: zppppppgkp
2-6 r: rvdrrrgdnrrrlrr
6-7 k: kkkkkkkk
3-4 x: nxxxmtmljgldjx
2-6 f: qfxxxftf
1-9 z: zzdlqgzwzzjw
6-8 n: rnjnstwm
4-6 w: vhfhwckw
3-7 k: kkpkkfpk
5-7 k: kkkfzkhkk
1-11 j: jjjtjjjjmjs
2-17 w: wlwdkkwmkwskpstxvrwr
5-11 g: kgpgjspqlwnx
2-6 l: lpllld
1-6 d: xwkvgvkdxlljpdqwmhs
5-6 z: llrzzsz
4-5 l: lllll
18-19 g: gggggggngggggggggggg
1-10 x: kkdxwkrtfh
3-4 t: ttrqzttbsrt
14-16 s: ssshsssssssssnsss
2-6 k: kjzkxkkhkp
2-10 b: bbbbbbbbbbbbbb
1-9 v: vvvvvvvvvvv
11-12 m: mqmmmmmmmmhsl
5-6 s: sssmsss
14-20 s: srssssssdsssskssksvs
1-3 r: rrrr
4-7 d: zddkdptdfdw
6-7 g: rggglggtgkgg
3-5 c: bcctcvpcprcccqccpcrc
8-9 r: rrrxzrrlmrd
12-17 c: hcctccccbqcctscsscv
15-16 d: dkddddjdddddsdhwdddj
13-14 b: blbbbbqbhbbbmz
2-10 p: hvxqpjqstv
1-4 p: plxp
3-6 z: zzdlzqjzzz
1-14 s: qsssssssslshswsjh
3-5 r: grrrnrrrsvrrrdrrrjrr
1-4 d: dfdjdxbpdddf
2-3 p: pppppp
16-18 c: cmcccjxfclcgmchswq
9-11 x: xxrxxxxxvxrxx
13-14 p: dpppppsppspftpppwp
15-19 h: hhhhhhhhhhhhhhhhhhhh
6-12 q: qqqxqqqqgqqpqqqqqqqv
10-12 g: gqgdgzhgghjgdd
2-6 p: pnppppp
3-14 l: lllllllllllllllll
12-19 h: hhdhhhhvxrfhcshhwxh
16-18 m: mmrmmmmlmmgmmmmmmmmm
8-11 t: tntmlpgfttv
8-11 f: mffffffffff
15-18 t: tftttttttjwtttjttjt
5-12 d: pzdrdqxzqzwgzwcgsv
4-9 d: cdwdmhksdzgd
9-17 l: mlsllmgdlhlgmfglr
1-5 r: rrcnr
3-4 c: ccccccc
1-2 g: cskvbgnlgnpfrh
5-10 n: tfnnvnnqbbn
1-15 g: fxpdggmtrqggdglh
3-4 t: ftttppdtdbxzqw
5-6 p: qpxsjt
8-14 r: bkltjzprkhrhmrg
1-11 f: gffkkmpflxlqfsf
1-4 d: vddknn
5-7 l: llllllvlllll
2-11 b: bbbbbbbbbbbbb
1-3 h: hhhh
13-14 q: jqqqcckhdqkqqj
4-14 p: mfwcbqxpdkhtdfj
3-4 g: ggggg
6-14 v: vvvvvvvvvvvvvvv
16-18 v: vvvvvvvsvvvsvvvvvv
9-13 l: llllllllllllllllllll
3-4 r: zjrr
6-8 x: xxxxxxxxx
2-6 p: zlgpps
3-8 t: ttjtbttjt
18-19 k: kkkkkkkkkkkklkkkkkk
8-13 q: hcvnkpnqfzqmqscwf
15-16 p: ppppppppppppdpmr
7-12 c: cqccccdcccwsccc
8-12 d: ddhddddgdddgdddd
3-4 x: xxxx
6-8 w: wwwrlzcbwhwlqwwwv
1-5 h: hhhhdwhhlhph
4-5 f: fffvz
3-8 c: ccqqcccc
3-5 k: xkwkdkh
2-3 s: ssssshhsss
4-6 x: gxxxcx
8-12 l: vsnlqbjltbtlphjtf
3-6 n: tgpnqxnntkmntvl
9-12 f: cjzkldhmvclk
4-7 b: zgpdznb
1-12 s: ssssnrssssscnssssq
14-16 p: pppppppppppppwpjp
4-14 b: xpbbvcbbbspgbbqbw
5-6 s: smssrb
5-16 b: tzbdqxgkbxbvzwvt
2-14 f: vzplvvftrcdmgs
13-15 t: tttttkthhtzttdt
2-3 s: vpwss
9-14 s: sssssssfssssss
7-13 s: tsnwzxwzjzsjkchlg
12-15 q: qlqbbfqqvqmcqhvq
12-13 g: gggggggggggggggggg
1-3 g: jjdqzgwgggggg
5-6 n: nlnxnnnwv
15-16 m: mmmmmmmmmmmmmmmjmmm
2-6 m: vmrqrmz
4-5 n: nvnnwb
13-15 x: xgblxvxmpskqwvlx
6-7 v: vvvvvvvv
8-10 r: fsrrdprbrfxhrdv
4-5 j: jsjjf
4-6 k: kkkjkkk
5-9 j: cflfvjjtjljjhbhnj
9-11 t: xswktxtqrbm
4-16 r: drrrrrrrrrrrrfwnr
13-18 p: bbkplmplqmppppspmp
9-15 r: kgnwvxmcrmltpcgwvrb
4-5 c: jcwcccnc
2-4 f: ffff
8-12 d: rvsdddvdtdcd
3-10 v: qvdzjhcvsz
10-11 x: qcmvlvgxkxx
6-9 s: sxsspsksjssfsnssrvs
3-7 x: zbxxxxxwrpxgr
9-17 j: hrppktjsqhzqwxhdz
5-7 z: nzzzvzswz
6-7 p: ppppsbpp
10-13 f: fffffffffhffmfcxffpf
9-12 f: ffffffffsffmf
14-15 r: rrrrrrmrrrrmrrzr
3-6 w: whwwwwwwwwwwm
7-8 b: btqbbbbbbbz
13-16 z: zznbbdmbzhjzzzzzmngd
6-7 q: fqmqltdqqqdwqss
3-4 h: hphhhhh
1-3 r: rdgktrmrmdztrkmn
17-18 w: wwslwwsmwswwfxwwwww
6-8 p: ppprpppp
8-10 z: zzzzzdzrztzzzz
8-11 m: tmldmmwxmjmrmtrmm
12-16 s: phpprsdvkmsssmssnqwm
3-4 h: qplnhkhh
14-19 w: wvxvqzqzdnjjwwtdbwc
2-4 p: fpnp
3-5 c: ccccpcccccccc
3-12 g: ggwggbgfggrxggg
4-8 r: rrszrgdbrw
2-4 n: vnmgjtps
8-10 j: mjjjxjjgjc
2-6 m: vmmhmm
3-4 w: vwww
10-18 v: mvnbqbvcpkvnhqvvbvzv
15-19 g: ggtggggggbggggxggglg
7-13 m: mmmmmmjmmmmmmm
5-11 d: ddxhdndtfdddndxzdqk
3-11 f: ffnfbfffffgrff
7-14 w: fflfpgklzlxfwwt
15-17 d: zxllzgbjrrddnmdhdwp
1-5 n: nnnnknnn
15-16 j: ffkjjljgndljjnjscs
2-3 v: nvvxvntqcrqq
5-6 c: rcccgpcc
2-3 j: qjjfrhwljgczj
5-8 k: kkkkkkkkkk
1-2 r: rrrrxrcrrr
1-15 t: ltttttttttttttttt
4-13 q: prlqzqvgpqslqqcqqjq
3-12 q: gfbpbdrwrbqt
2-5 q: qsxccx
4-9 t: slglllcdkjzdmxt
2-3 r: rlvrrr
2-13 w: wwmkwwwjcwbcwzwwcg
3-4 t: ndkvx
6-13 b: qflzbbbcfzkbmtx
6-10 j: pdckwfhkjxqljnjjw
1-5 j: jjjjjqmj
2-10 x: xlxxxxxxxfxx
1-5 r: rvcnr
4-10 c: cnchrcsgcvncpq
3-5 j: fjxbj
4-11 c: ccdlcsxckcx
12-16 m: mmmmlmvmpcmnmmdbmms
3-4 q: qqwlqqqq
2-6 f: kvcfdv
8-10 m: mmmmmmmmmm
7-8 j: jjjjjjjj
5-7 c: ccccncsjc
7-9 f: fffffrpfplhzfff
1-6 c: sbcccscccccccccccc
5-7 j: njpjqxjrljnqjwjgchtj
11-18 b: gktmxhjwbmbpjrbqkb
1-4 f: lffbfffffw
5-6 c: ccccsc
10-14 v: mkbxvxvgvvvvbv
5-6 x: jxxxxxx
4-6 c: qccmhcccw
3-7 s: qwmtwprsxd
10-19 d: rkdwbrndmxdsknddddl
2-5 p: pppppppppppp
6-7 w: wwffxwc
2-7 z: pkzcnxr
5-10 b: bbqbkbtdbbbbb
10-13 j: qmqgvhzkjmfjhjjm
5-6 n: nnnnrn
7-14 n: qgmlsvnjnrdzcdbstxv
9-16 q: qpllvqpmqlqvkhwqqrsm
2-4 s: sqpnqsgsssq
13-15 z: zzzzmzzzzzzzzzzzzg
10-15 b: bbbdbmqbkjbbtbd
2-5 h: qrhhhhp
11-12 x: xxcxhmxxxxxx
13-18 c: ccccccccccccdccccccc
12-13 h: knktllrdtgjhhhxpd
9-13 g: tdpfvzcqgbslgnbxkgpq
8-15 g: gggggggsggggggxggggg
1-3 v: wvfvvdvvvvv
10-11 b: bbbbbbbbbhzb
15-17 b: bbbbbbbbbbbbbbpttv
1-7 m: bmmmmmdm
2-3 h: rhksbfgh
6-7 w: wwwwwgf
3-12 h: hkhzhhhhhhhlhhmh
3-5 x: xxcxpx
7-9 m: mmdmtmjdxmm
1-4 n: lnnlnnnnn
4-9 k: kkkmkkkkbk
3-4 t: tttt
5-12 k: tkgwkrwwzcwkhkmk
4-5 f: fhfvfgwfff
3-5 h: fmbgjrxgh
5-6 j: xtwnjj
11-15 r: rrrrrrrrprqrrrfrr
13-14 f: fdffffflfffvfff
1-6 b: krsbnd
6-7 b: bbbbvxqbl
1-2 v: vvcvvvv
7-9 t: ctfcmttht
2-7 v: bcxhtdwbfqvv
14-15 n: nnsnlnvkqztnxnn
2-6 g: rknndvrhbg
2-6 z: mzzzdzzzz
11-14 q: qqqqqqvrqqzlqkqqq
3-4 t: ttcrttjtknjpvwhzm
8-10 j: jjjjjjjjjjjj
2-7 c: ccnhcccc
3-14 m: mmmmmmmmmmmmzmm
10-16 w: swsmwwwwwrnwmwwk
4-5 m: bmdjm
6-11 l: cmlllxllllkrllf
14-18 t: tttttttttttttvtltt
11-13 f: ffffffffffhfzfff
3-6 g: wkzxtrkgr
7-10 c: cccpcccccvccc
3-13 m: mmbmmzbcmlwmmfnmmq
2-3 p: pdszp
13-14 r: rqrrfrdrrrwvrrrrrwrr
9-11 g: gggwggggfcpg
3-9 p: pnppppdpppp
17-19 k: gzkktkxkkskkkmkkkpk
4-11 x: bmqhtxzqmvx
11-14 v: cvsvssqghzjqxm
1-14 z: zwxczzszzdmkztl
8-9 t: jkpptwctt
3-8 z: vqmzzpjj
6-10 s: msdrxssssswssjz
1-7 z: zhpzzzz
13-17 m: mmmmmmmmmlmmgmmmgm
9-10 n: qgtcnnnnnkcnnn
2-4 v: vrcvv
4-5 h: hhhdwh
8-9 c: zcccccccc
13-14 p: pjxcpmdpppmgwt
2-6 c: gskmcc
6-7 q: qqqqqqqq
3-13 b: gfbxqtnxnqxbb
8-10 l: lhrlqlrqlsllll
5-8 z: jlzgzzzq
10-13 z: zzzzzzzpzkzzgz
6-7 d: wddnddd
10-11 h: vbhhbhhhhbz
7-10 s: sxssksskqs
1-12 b: vdgxvsxpgbbpd
5-6 n: npnvckn
3-4 v: vvjvv
9-15 r: njhhqlmcmncjrpx
2-8 h: bghrdhhrjbhhhbjz
5-7 q: qqqqlflrq
9-12 f: qfhfffffffffffff
2-3 m: mmmblxmbpz
13-16 v: vvvvvvvqvvvvlvvw
6-12 b: wbfgbghzbmvf
12-15 r: rbrrrrthrrrrrrrr
4-8 k: pkprqkkkpsphwnhp
8-9 h: hhhhhhhsl
1-3 b: bsbn
15-16 j: ktjjjxjfjjrjjcjk
4-8 z: smzzmzzszzr
3-4 m: qmmmzqzlrpt
11-12 n: nnnnngnnnnwgnnnn
5-10 x: slpbxqfxxxx
12-19 q: qqqqgqqqsqqqqqqqsqqq
4-10 s: ssstssssss
6-11 g: gggggvggggzggggggg
12-14 c: cckccccdccxnngccc
3-5 j: jnrjvj
2-6 t: vtdtms
13-15 p: ppppppppppppjpq
1-9 j: jjjjjjjjjjj
8-9 d: wfdddddkddxd
3-7 p: pwpcpvp
3-4 h: hhzl
9-10 n: lnkjxnsbnn
7-11 z: zzzzfznzbzbzbzd
1-2 m: mmzlmkm
6-7 t: jbmpztx
5-6 v: vvbvvtn
7-17 q: lvdfpbqxbxdnwqzjlfs
3-5 z: zdzzzxczbzzz
10-14 f: fpffffffffffff
3-8 x: wxxchxjx
1-6 x: xxxxxx
14-15 x: zmxtcbjxtvxxxxg
4-5 f: qfffghf
2-3 p: pppp
10-12 n: nnngnxnnnhnsbntln
1-2 w: ftwwww
10-11 f: fffffffffpkf
4-5 n: nnnnnn
10-11 r: rrrrtrrlrrrr
5-6 h: hjhxhhnbhlbxvhs
1-4 q: qqqq
7-11 p: dkpfppwzpxqqx
11-17 l: lzknllnllkgvllllh
1-3 z: zzzzz
16-17 b: bbbbbbbbbbbbbbbbbb
2-4 n: qcpwnhn
1-4 v: svvhvvv
10-12 l: llllllllllljl
1-2 l: xzlllllll
1-3 l: plklll
4-8 x: xxxxfxbxcxxqxfxpx
3-8 b: xgbbwncxclbv
5-6 m: mmmmmmm
4-5 p: pptppfnpl
14-16 x: gwjfkzwrldzxpdxgk
10-11 z: zzzzvzzzzzzz
3-12 k: kkxkblkkkkkgkkkmkk
4-6 p: pppfpx
2-7 r: rrmrrrrrrrrrr
7-9 p: pbpfpppppp
2-3 v: tvvbbjnvv
5-8 j: jjjjjjjjj
2-5 c: xcwcczjkvgccccvpkd
5-7 f: ffffpfqfffff
11-13 m: qmmmmmmmlwkmmmmmm
14-15 d: dddddddpmddddszdd
6-15 b: kptphpbxxqsbxlhxplml
16-18 x: xxkzfxgqxdcvzdxlzn
10-12 m: clrmmmmksmmm
1-3 w: wwpww
8-15 g: ggggggnlgggggggggng
3-6 h: tnhhxh
4-5 l: lllrdllxlnllqll
1-2 m: lmjhvl
1-4 k: dkkhkkk
6-7 x: wncxcxf
4-5 w: wwwhfww
4-8 b: bshnbbdbmfxbbrc
4-7 t: tbdtztm
7-8 n: zntlhccn
10-14 f: frgffffffflfzlf
3-4 b: nzbbbnfbdg
2-6 m: srmwsmtp
5-6 z: zzpvsrz
2-13 m: msmmmhmmmmmmtm
3-4 c: ccvcc
1-4 n: nnnnnn
2-3 j: zdvsj
4-6 c: ccctccc
4-5 z: zzpdztnrzl
4-6 z: hzdzpz
2-6 x: gxjxcxxjqcb
4-9 j: jjjvjcgzl
5-8 z: dmppvjqfzzzzn
5-7 v: vvvvvvr
2-5 l: lvkkft
2-11 n: nnhnqnnvnsn
1-3 d: dhndddjfd
8-9 x: xxxxxxxxxxxxxxxxxxxx
4-9 p: fntpgcwtwv
4-12 q: qbrlqzqcnwtjq
2-3 f: ftcjvnfkzx
1-2 d: dddd
7-10 g: gggggghggzgg
4-6 g: zhgbgphggtm
2-4 l: lblq
14-17 n: bnnngnnnnnnznnnnn
5-6 f: fxxffffhmswq
6-10 p: pppppppppp
2-4 q: lsxgqkqsblqqq
11-19 x: xxnrgxxxxxxxxxfwmxx
4-7 j: jjjtjjljjjjjjj
6-7 l: llllllll
1-4 f: pfftffffnffff
11-13 r: xrrlrrrrrqrrzrr
1-3 p: pvpqp
9-13 d: ddddxrnddddfdscsd
1-2 f: lbbr
3-7 t: mpttsgwb
2-10 c: jccchctshcpchwx
2-4 p: lmpd
1-2 n: zlnxnnf
2-3 w: fwwtqw
9-16 s: sssssssssssssssssssj
1-5 s: hssrm
15-17 b: ghpvwjbzpksvhsjbj
8-9 h: hzrvxhhdz
3-4 j: mkjjpvkfhprr
1-8 n: cnnnnnnmnn
1-4 x: lxxxx
3-5 m: pjttcvvcmfvzffcfmmv
3-10 r: rsxtvwjrfrqrrbzzr
2-7 h: hhckhhr
7-8 r: srrrmprrrhkzsndrrkr
4-13 m: pdsmlsnxcmhmmsvc
3-12 d: ddsdddddddddddddd
14-16 s: sssssssssssssjsds
3-4 m: djfp
2-5 w: bgwpc
4-5 r: zffrrqrs
4-9 r: rrrgrrrrqrrrrrrr
4-6 w: wxwwvw
7-8 d: tvddddwtdddldd
6-7 g: tgdggggfzrgqggggmggg
2-6 t: tgtfdtttttttttqxd
14-18 f: fffsffgfffffskfkff
1-12 b: qbvbbbbmbsqbcbgbbbcp
5-7 d: ddbqhdddsdm
6-9 n: tqwnckjsn
1-3 n: nnnn
5-10 w: wwwwwwwlwwqwwwp
3-4 r: rrrr
2-12 m: mmchtqmhfdpm
6-9 d: dzgzdbddwgdwldb
10-14 k: kkkkkkkkkkkkkzkkkkk
13-14 v: vvvvvvvvvvvvvvvv
2-12 f: btqtbrvkmfzhz
1-5 b: bbbbbb
7-10 j: jhjnjjwnjbj
2-16 h: kjszvtwrjgrvzrqlzcb
6-8 t: nxzftvtnrtxg
3-6 h: cvhhnhnhhcgst
5-8 f: fxgfjnjzz
11-12 b: bbbbbnbbbzbqbbbq
1-6 h: hjzhhw
18-19 m: xrvmftmpkpmrlkqmxvm
6-7 v: cvxbdsr
5-6 v: vjvvvvf
2-3 m: mmzpm
3-4 k: mkghkxc
1-6 s: sjsmjsstsss
2-10 s: wndvkxbmffh
9-15 q: qqqqqqqqqqqqqqqqq
4-5 f: rffffpc
10-11 v: vvvvvvrvvlmv
6-7 n: tnnnnlq
16-18 g: gggggdgdgggggggggggg
5-11 l: lwlfvzlsjll
11-14 v: vvvvvvvrvwvvvv
12-16 j: jjjjjjjjjjjpjjjx
4-6 w: wwwwwr
2-4 d: ncdd
15-17 c: nccccctccwccccqcqcc
3-5 m: mnlmqtsvn
7-8 w: swwwwwnl
6-12 l: zlplklqldllxlpvb
7-9 h: fhkhqnfhhzrhqhhh
11-14 v: vvvvvvvvvvvvvv
16-18 v: vnvvsllmvswcvzqvvxjl
7-9 k: kkkzkkkkskkkzkkkjlk
5-17 w: wckwxwlrlwrncxwwb
2-4 v: vqvvqgx
2-3 b: bbbbbbmbbbbqb
4-5 p: ptphbprpv
3-6 q: kqqkpqbnt
15-20 n: nnnnnnngnnnnnnnnnnnw
10-12 p: pppppxpppmpnjp
5-10 x: bhwlxkrdxxqvkphvmfgn
2-3 l: lllhql
4-11 l: wllvwfccllk
7-11 d: dmvmmzlngpw
2-6 v: rskbvv
7-9 k: kwtckkkkx
11-12 j: jnjjfzjjjjjjjj
2-5 v: rvvnnjjxjnnt
2-6 j: ljcqpjmhgmmcxjkgd
1-3 k: lklbx
14-15 c: ccccccccccccckc
2-6 x: xxxxxxxxxxxx
1-7 t: btrtxtjgtt
2-3 t: sttt
5-7 k: kkkkzksxgb
3-4 q: mnnxqwtnw
8-10 t: vwzmnxjdzx
4-10 f: svglffffpzw
1-5 s: dsfjsv
8-10 m: vmwpmmmjmv
6-7 h: hhhhhwxhhhhhhhhhhhhh
7-19 n: ktlblznjctnnrzwrknt
3-6 n: nzxclnvc
2-5 t: wtrttkx
9-10 j: jjjjjjxjjj
12-13 r: rrrrrjrrrrrwx
11-16 c: plvcnjcpnkvcpcmcpbc
2-3 c: gccclcslqdmsg
10-13 d: dddddddddhddd
8-9 w: kklwmwqmwwtwqw
2-5 q: qsqqwx
3-4 c: hcqvfbccccctcc
6-7 k: kkkwqgv
13-15 h: qhhmhkjvhhhhngnhhhhs
4-5 d: msjdd
6-7 g: ggggggg
5-8 r: cqrdrfxnrrjrsr
9-15 r: lprrqrrrrrrrrrrbr
6-7 h: hhrqhrlhhh
2-3 g: glgg
1-11 n: nnnnnnnnlnnjzwnnnn
3-6 l: bwllrl
2-6 p: nptphqv
3-8 l: lrlknvlpcm
2-8 g: wjbgpjrlfsgg
5-6 w: swwfww
1-8 p: pptpxppppppwpppfx
3-4 d: wlddg
15-17 c: sccccccgccccccccc
4-6 n: nnnhnp
8-9 r: prvrxmprlzxs
9-12 t: ttttttttttttkrtt
5-6 h: hhhhkfh
1-10 q: psqpzpkqtcq
4-7 f: ffflnfzkf
10-11 j: jjjvjjxfjjj
4-11 p: wlhgrxlclnt
1-14 t: wfntvttxmldhqg
14-19 c: ccpczqrccfhtcvljlvc
11-12 w: wwwwwwwwwwwww
2-8 z: lzsdqxzz
17-18 p: pppppppppppqphppprpp
2-4 j: wjjq
4-6 f: ffkfffgff
8-20 x: phxxxxxnwxxdqzxxxxxx
9-10 d: xvddddsldd
3-10 l: rglllxlnfl
6-15 d: ddddddgddpddddwd
6-7 w: kwwwwhs
8-10 r: rrrfmrrrrd
2-11 x: xcjcgtkzkhtmrxqjxxx
4-5 v: wvmdr
7-8 d: dddddddd
7-13 l: lllllldlxllln
13-14 z: zzdrzzzffzzzzzznzz
3-4 j: zjjj
8-9 s: sssssssbss
6-15 f: pfsfvfxqpffqfqf
9-11 p: pjpppprzpcphvppwp
3-4 c: ccrd
9-10 c: qcwbccccjhwvtwcccnc
3-4 x: xgxx
5-6 x: xgxzxxxxxxx
5-10 l: kxdgnqrkpqgcmcmnk
4-7 v: vvvpvvvv
1-4 n: nqnnnnnn
5-6 p: pprgpst
14-20 q: wlxrjczhxwdctvpcgxqc
5-8 g: gcggbggggggzgg
10-11 g: ggggtggggggg
14-18 h: hhchhhmhhbwhhlhsvrh
3-6 f: ffmfffffffff
1-12 n: rnnnnjwnnnnv
7-12 l: bpxlhthjplljwxvvvjm
4-5 j: jhjjjjj
9-12 q: qvqqqqqvqsqq
3-4 d: ddnvdddddgddd
1-7 h: xhhhhhkh
1-4 f: fffff
3-4 q: qfrq
5-8 k: kkkkgkkqkk
14-18 k: nkkkkkkfkkwkkkkkqk
1-4 q: qqdc
17-18 f: ffffffffhffffffffff
16-17 b: bbbbbbbbbbbbbbbdm
17-18 z: zzzzzzzzzznzzzvzmwzz
3-4 g: tggd
4-6 t: mlttlqqz
11-18 f: fgffgwlffffftfzfrt
7-9 h: hmmhhhpljjn
16-17 m: mmrsnmmzfhtmrjwmmjm
2-3 l: cllcbm
3-4 k: fkkkqbpbmmcd
9-10 f: kfmchwkwzfh
5-6 j: jjmjjjjjj
3-5 v: vsvbfvvh
3-4 k: tgkk
2-8 k: lscvjjnhpxl
7-8 v: vvlvvfkqqv
8-9 q: qcqqtqqhqvqqtq
12-14 z: zzzhzwzzqzzzzwzzlz
5-8 k: kzknkkkkkp
5-6 s: spsssw
2-3 k: kmkh
9-10 f: vnlxmstqbsg
1-2 k: kkkk
6-15 r: rcrrrsrrrrrrrrzrrqrr
3-8 c: cfczjwkcxbc
15-16 k: kkkkgkkkkkkkkkkh
5-18 c: ccdcxtccccccpcfccv
6-11 w: qdwnfhzlwwwwxwwgwhwk
1-6 d: fddddddhddw
7-10 p: wpbppvpzwc
12-14 k: kkkkkkkkkkhhkslsk
4-7 r: hdrrfhrqrdmcbnlrrjbr
7-10 b: bbbbbjbbbbbb
4-7 p: pzppzppsp
11-12 h: qhhhhhhqhrlwlhhthh
6-9 s: sbssxsskswvp
17-18 b: bbmpbbhhbnsbbbbbbb
8-9 g: wgggktggg
1-12 s: vsssssfssssrqk
2-5 s: bsgssxc
3-4 c: ccccccc
2-3 d: dddd
5-7 r: ldlcjrrlrngr
5-10 m: msmjqmmmmm
3-7 c: ccccccccc
13-14 p: pppxpppppppppcpppp
2-3 r: jrfrvrk
11-12 g: gggggggggggg
6-18 t: ctvqgcrgnxdvbzjfrrbt
6-11 f: rnjptfnwgxfp
5-8 w: wwwtwwwwww
14-15 r: rkrbrvrrrgrczrz
12-13 t: trtjtttlnxnxx
5-8 l: qnwllfsl
2-15 g: xgtcjftlqqfwkggpf
11-16 j: jjjjjjjjjljjjjjjj
8-9 b: bbzbkbbvgcbb
5-6 r: dvkxrrsvrrksszsdr
12-13 j: jjjjjjjjjjjhdjjj
16-17 z: mzzzrxfzzzzczzgzz
2-15 p: lpjxcdzjmnghfppr
9-15 s: ssssssssnsssssss
1-11 t: tfvtqvlbtld
4-5 k: kkkczkkkvkkk
2-7 p: ptphppvppppp";

