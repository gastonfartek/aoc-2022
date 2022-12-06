use std::collections::HashSet;

pub fn part_a(input: Vec<char>) -> u32 {
    let mut i = 0;

    while i + 4 < input.len() {
        let set: HashSet<&char> = HashSet::from_iter(input[i..i + 4].iter());
        if set.len() == 4 {
            break;
        }
        i += 1;
    }

    i as u32 + 4
}

pub fn part_b(input: Vec<char>) -> u32 {
    let mut i = 0;

    while i + 14 < input.len() {
        let set: HashSet<&char> = HashSet::from_iter(input[i..i + 14].iter());
        if set.len() == 14 {
            break;
        }
        i += 1;
    }

    i as u32 + 14
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    const INPUT: &str =
"srlsrsnnwhwwmddwfddgldlglppcnpptftzzchzhbhmmvwmvwvvjsjnjbbvbmvmnmqmfmjfjjmllzddrvrrnsncnznndwwqrqjjsfsjsjsvsvzvtvfvrvhhbwwltttbhhvphhtqqnffsppdqqrmmfrmfrrwvrvrsvrsrzssrqqrfqrfrttqntnpttvrrcncnfftjfjwwfwjwpwwltlwwwvnvcnvvdgvddqlddnllrggqvgvzzjddfgdfdgfffsbfsffjmffprrmgmtmqqgzzlwldlbbgjjnfjnnvlnnlbbnjnzngglmggwtgwttnvtnvndndmnmdmcddzbdbbspbpwbpplvpvnnmtmqqwnwgglhghdhbhqbhhqrqhqcqmmmrwwlvvfnvfnnpddbfbgfgqglgddrggbgbqqfwfjfdjfflzldlbbsbbtftccgqggzgdglglzljjzbzfbzffgftfvtfvvvjmjzjmjfjqqccpcjppzfpzzsvvzjvvfgfmgffzhhdphdpdwdwmdwwlqljjzffzvfzfwzfzcztctssdpssndssvbbtbstbtnngjjspplttpffjggbbqsqnqcncscpcmccbmbdbtbcbbgqgwgwqgwgwsszqznzvvntvvmlvlvqqvgvwgwlldbblclqcqggtqqlsqlsqlqcczzqgghmgmrgmrggljlbjbsbsmmnlmmdppnpfnnvhnnzggchgggbwwvtvpptgpplsljjshhnphhblbnbpbgppfllhsllbtlbtllrprqrvvfgvgpgcpcdddjjplpssjrjqjggncnzccqdqfqpffvddhlhdllgzzrtrztrztzptztwwbjbnjnsjsfsbfbdbccnhnmhnhrhwhfhbbtzbtztzrttrzrfrcrtctqqrlrsshphwphwpwwcmczzpqzqztqqvzzshhddrvdrrztrrqcqbcqbbhnnpfnpfnnmcnmnccnmmhccjjwzwcwbwcwnngtnngrngnsnzzcvvvtnvvtpvtpvtvvhffbsfsnsdsmmzbbrqbqpbpcpdppzmzbmmtdmttvgvrvlrvvhddjgdjdbbthtjhjdjhdjjmssdldjllwppvbvnbnlbbbszbblpbpfbpblbzzszvssbcbgcghgbbcpbbqrrdjdnnjcjddjcdjjcqqhggflftftfhttvlttgdtdrttwppwvvrzzbggrjjmssrjsjmssdtttdbdvddpllpgllthhvfvhvddrhhnznwznwzzpbblvvpmvpmpddjvdvcddvsshfhrrghgnhghmhhmhffrsfrflrfrjrddzgzsggchcfhhsbbsrszzgjggnrrgfffhthdthtnhtntllsjjjwqqsbsttbtzbttlfttczzbfbwfwvvhllltjjsqsdqqspqsszqzhhdrrhccgbcgggcbggnvgvmmwcmmpqmqccshsnnnwrwfrrwrllvhvlvmlvljjgrgvrvddfqfwqfqlfqqqppvmvrmrfrvrwrqrjqrrvmrvmrrvffmtffsfdfdrdwrwjrrcgcwwcdwdrdnrnzztqzzzgdzgdgldltdtvtnnhllrtltbtrbbttgssdhsssrmsscnssmtmqqtssmzzjqzqddfrdrtdrdpdzzppwggsrrfvfnvvmgvgmmdnnbwnntlnntncchcmcjmmvzzzngznnbgbnbnvbvzzfqqmtmrmcmqmvmbmbmcmscsjcsshswsjsvswshwwlnnsslmslmmzgmgsmssrwwmjwmwllvbvppplmmwgmgzgzdgzdgzztrtzzchcssbppdbbmvmqmmzhztzdtzdznnqhhmcmnnncbnngjnjmjnmmqqsnqsnslnlqlppvbvwwvtwtfwtfmzsbdjfvhzcflstpbtprpzmwcwwgzshfssnjdgflcsfglhbvvdctfrccfmwgfvjtvqfqpzzmqtlbvrjqwlwclfzhfzcmwsvprdhzcrghdscprrnqgdgmhhwbcfcfzrcgrwvbstpwbcvcnpmftwwthnsjznzslfdfqhtpsgmjsddmhlhtlzfsnhlcbgnqpqbzppnbcgtzvwdpmgftgsmrwjqwsdnjrrmwrmlcvlqgwldnvmsgnwccwzmdzwdhpmpqffgbnrrrvzqhtlnvqfhgwbbwsghwbrjzjcjmjmsspcmnfbbpmwffbdqffqfwsvtcrghbnldspdhjhjvtdhgmvrzqqlhvnwbdhfprwqdtzswfdzdwhzvfzsfqlzcflhbsvjhnjnthrdrgmjrrbftflpvrmrvvsgnjhrhjffvznmbjtshwgjmcqbfqjlptsrcgzvrhctwsdclrstmsbsbpqpjcvftmvmsmpwmmphlwmljhvllhrmfcncdprrwfmqrlllmsrvrwtbsqddbmblhvqqlcbbtjbslggrshflrddsphbvzhprcvbpmfldnmwtchddbdwhwpgbmjqptsbdzhrbqfwgfltchsggjlbqbhmzbjdncnvsqjvftpwrpdcjzsrvrlzwqfptwzssqpltjhdmtmmgvjzzqswwrsqrqfpdllgzsdlvhfdnzlrmlqdclwhqqbtwdqcrlfdjphjvcfzpwwgphmrldnwjpnmsjnqpwtwglcbthmcbnvpmjbdhmjglgccsmrmbvbqphdlbrrgnbljlhqvgqbctmthvmsfzflwvctwmdgmrlhnmwnsjtzndlwfttcvtslppjwsgbzhztlpzhmjlrbtzfnblwwlddzwqvzbjmffbflvmvnsgjlqwjqvrcrtwbhfzbzzmtwsndqmtnbjbsqjpnwhzqgwwltwgrqfbhjtpzrqvvhgjsbzqmfdsmzjdnplvblvqlrtzsbjgdptdhgvvrblbjppnnhhnvqtmjgwqzwtmjglrjlwsrfdjfmzlsszzzpbwvjbwpcwlplhcmhgpclztjcrmcntnqbnlrmcnhcnhnmjggtqbtvcfmcbdpqhdrvspbrgcwhgjhggwpvjbplhdsnnvgbzhmcrjqvqlwsdvrrnntmwdgdqdvzznjflctlfmfzsdfbpftsscnbtnqcsvtltvnsqpghzcwtjctvwqqnvnddlzbftwrdqzltpczrbmvsmlpfjzphtpnngzdfrjqrtbppbvztsddblspbltsnrtdftmwbblgdqvmscjqpwfvdzphwqgrbpdfcvtvmddgjjpswrqnwjfghtzwzbhtjnwjsrnwhjwcwnrtwrmbmqjhrbmbsslnvtsqnlbzlfdjvmnlhccpzmvpmrnttnshdjsswjlbtpwdwgmzqcthbczngwnfpzbmmrdffqsbrmsgzrtpsstfmdgbclwthflfbzvtptdfpzznvdwncqzvjwsmczwcnwgjpffcbhrtmtnbtjgcvtwdjtrftjwgvzmlfvgdjjjgsftzctplmjjmqfdzzwrtfmwqhwzglwbvsdvtgphsmngvppvlfdjdrzhhzsrvqgfwjbqdqlrwbgdmrglzrchvgwlhhwbvhfszntbrhshvqdznfvngdmhctjnlfhsmchqbqhdtmlntsnpvdzrvwjcjflfqgnprcpscfcmlgbwzmbnfdfnbwngrrbpnvbcdfgcpgtwmtbzptsdvjhwnzpsffglnqlczgsrdzshrfpqtqbmsrgzwwjvqhzjfzldcggvfhtcvzsgfspnlgtjfglwcnbprvssdfbtzqbblzjzmtftfcfprlwsvswjvtpzcpfphvmdnbczzjbjjrpwrncdszrtqjvznvgzcwdvpfvfcqlsdmhsswpnfsvtffnjhstcznwghlpmmqbnhqtddbjdrpvfhgdwjcnfgggmwnqhmsvgzsssbfcfrmbwpfqhrmtdvhbpwzggdvfjnlrgmvjjbqhjmbvszfzdcwrcjhfdczbhmcrwngmrgrldvbjjdgddhbbvqvwztfstfzqnqwvmmdzhzqtrmnqfrhmtdfhrgfqclpqdwthrbgsprccwcsvdbnclglgflhtqstvpwrmnsvflpplrgftlvjwttmdcqwjnbttjqqlwsntplmfrncqzplwgrjpzljtmnwsqfqnzgnrbtscbfzqwnqttnghcgl";

    #[test]
    fn test_sample_part_a() {
        let result = part_a(SAMPLE_INPUT.chars().collect());
        assert_eq!(result, 7);
    }

    #[test]
    fn test_part_a() {
        let result = part_a(INPUT.chars().collect());
        assert_eq!(result, 1876);
    }

    #[test]
    fn test_sample_part_b() {
        let result = part_b(SAMPLE_INPUT.chars().collect());
        assert_eq!(result, 19);
    }

    #[test]
    fn test_part_b() {
        let result = part_b(INPUT.chars().collect());
        assert_eq!(result, 2202);
    }
}
