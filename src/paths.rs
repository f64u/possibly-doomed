use std::collections::HashMap;

use lazy_static::lazy_static;

#[derive(Clone, Debug)]
pub struct Path {
    pub path: &'static str,
    pub text: &'static str,
    pub cta: &'static str,
    pub choices: Vec<&'static str>,
}

macro_rules! path {
    ($path: expr, $cta: expr, $text: expr, $choices: expr) => {
        (
            $path,
            Path {
                path: $path,
                cta: $cta,
                text: $text,
                choices: $choices,
            },
        )
    };
}

lazy_static! {
    pub static ref PATHS: HashMap<&'static str, Path> = [
        path! {
            "/end",
            "The End.",
            "You basically lost, since you broke the dt that asserts the Prince's and the Princess's union. Start again from the beginning.",
            vec![]
        },

        path! {
            "/",
            "Birth",
            "A ____ is born.",
            vec!["/prince", "/princess"] // one done
        },

        path! {
            "/prince",
            "Prince",
            "You are born to an Egyptian King after he had trouble conceiving a son. However, your fate was determined at birth by the seven Hathors so that you die either by corcodile, snake, or even dog. Hearing this, the King locks you up in a stone house in the desert and forbids your leaving. The days pass by. You have never seen the outside.",
            vec!["/prince/stay", "/prince/leave"] //done
        },

        path! {
            "/prince/stay",
            "You stay locked up in the stone house.",
            "You stay locked up in the stone house. One day, you see from afar a man walking with a creature. You ask your servant what it is that the man is walking this, and he answers that it is a dog.",
            vec!["/prince/stay/ask-for-dog", "/prince/stay/no-dog"] //done
        },
        path! {
            "/prince/leave",
            "You ask your father to leave.",
            "You ask your father to leave, but he refuses since you're too young.",
            vec!["/prince/stay"] //done
        }, // End of path

        path! {
            "/prince/stay/ask-for-dog",
            "You ask the servant to bring you a dog.",
            "You ask the servant to bring you a dog. Even though your fate mentioned a dog ending your life, you have been locked up in the stone house for too long and you feel lonely.",
            vec!["/prince/stay/ask-for-dog/pet-dog"] //done
        },
        path! {
            "/prince/stay/no-dog",
            "You nod, cautious...",
            "Even though you have been locked up in the stone house for too long and have been feeling lonely, you're aware that, according to your fate, a dog could be the end your life. You stay away from dogs.\n\nThe days pass by and you grow more wary of your way of life. You accept your fate and ask your father to leave again. He agrees.\n\nA chariot is harnessed for you with all sorts of weapons and a servant commited to your safety. You go northeast towards Syria, where you know the Prince of Nahrin lives. After many days of travel, you encouter a gathering of boys leaping, seemingly trying to reach a window at the top of a tower. They see you, and they ask about your identity.",
            vec!["/prince/stay/no-dog/lie", "/prince/stay/no-dog/truth"] //done
        },

        path! {
            "/prince/stay/ask-for-dog/pet-dog",
            "You pet the dog.",
            "You pet the dog and raise it. He seems to like you.\n\nThe days pass by and you grow more wary of your way of life. You accept your fate and ask your father to leave again. He agrees.\n\nA chariot is harnessed for you with all sorts of weapons and a servant commited to your safety. You also take your dog with you. You go northeast towards Syria, where you know the Prince of Nahrin lives. After many days of travel, you encouter a gathering of boys leaping, seemingly trying to reach a window at the top of a tower. They see you, and they ask about your identity.",
            vec!["/prince/stay/ask-for-dog/pet-dog/lie", "/prince/stay/ask-for-dog/pet-dog/truth"] //done
        },

        path! {
            "/prince/stay/no-dog/lie",
            "You lie about your identity.",
            "You lie about your identity, trying to evade you fate. You say you're the son of a chariot warrior from Egypt. You say that you left because your stepmother has been mean to you. They believe you. They embrace and kiss you all over your body.\n\nThe days pass by, and finally you ask the boys what it is that they're engaged in. They tell you that the Prince of Nahrin has promised to marry off his only daughter to anybody who reaches her window at the top of the tower.",
            vec!["/prince/stay/no-dog/lie/leap", "/prince/stay/no-dog/lie/wait-until-healed"] //done
        },
        path! {
            "/prince/stay/no-dog/truth",
            "You tell the truth about your identity.",
            "You tell the truth about your identity, accepting whatever it is to happen to you. You say you're the son of the King of Egypt. You say that you're doomed to three fates, a crocodile, a snake, or even a dog. They believe you. They embrace and kiss you all over your body. They ask a guard to inform the Prince of Nahrin of your status. When the Prince finds out, he asks for your presence. You tell him what you told the boys. He tells you why the boys have been leaping outside, that he would betrothe you to his only daughter if you could reach the window at the top of the tower where his daughter lives. He urges you to do the same, since he would like to marry his daughter off to the Prince of Egypt.",
            vec!["/prince/stay/no-dog/truth/accept-offer", "/prince/stay/no-dog/truth/decline-offer"] // done
        },

        path! {
            "/prince/stay/ask-for-dog/pet-dog/lie",
            "You lie about your identity.",
            "You lie about your identity, trying to evade your fate. You say you're the son of a chariot warrior from Egypt. You say that you left because your stepmother has been mean to you. They believe you. They embrace and kiss you all over your body.\n\nThe days pass by, and finally you ask the boys what it is that they're engaged in. They tell you that the Prince of Nahrin has promised to marry off his only daughter to anybody who reaches her window at the top of the tower.",
            vec!["/prince/stay/ask-for-dog/pet-dog/lie/leap", "/prince/stay/ask-for-dog/pet-dog/lie/wait-until-healed"] //done
        },
        path! {
            "/prince/stay/ask-for-dog/pet-dog/truth",
            "You tell the truth about your identity.",
            "You tell the truth about your identity, accepting whatever it is to happen to you. You say you're the son of the King of Egypt. You say that you're doomed to three fates, a crocodile, a snake, or even a dog. They believe you. They embrace and kiss you all over your body. They wonder why you have a dog with you if you're fated to possibly die by a dog. They ask you to kill the dog before it kills you.",
            vec!["/prince/stay/ask-for-dog/pet-dog/truth/kill-dog", "/prince/stay/ask-for-dog/pet-dog/truth/keep-dog"] //done
        },

        path! {
            "/prince/stay/no-dog/lie/leap",
            "You leap to reach the princess.",
            "You try to leap to reach the princess, but you have not yet recovered from the weariness of travel.",
            vec!["/prince/stay/no-dog/lie/wait-until-healed"] //done, because other
        }, // end of path
        path! {
            "/prince/stay/no-dog/lie/wait-until-healed",
            "You wait until you heal from the aches of travel.",
            "You wait until you heal from the aches of travel. Now you sense that you can engage with the boys trying to reach the window at the top of the tower.",
            vec!["/prince/stay/no-dog/lie/wait-until-healed/leap"] //done
        },

        path! {
            "/prince/stay/no-dog/truth/accept-offer",
            "You accept the Prince of Nahrin's offer.",
            "You accept the Prince of Nahrin's offer and try to leap in order to reach the window of his daugher's chambers at the top of the tower. You succeed in doing so; you reach the princess. She kisses you and embraces you all over your body. You eventually get married.\n\nNow, the princess already had a dog that was brought to her to give her company while she stayed in the tower. You remind the princess of your fates, and that the dog might be your own end. ",
            vec!["/prince/stay/no-dog/truth/accept-offer/ask-kill-dog", "/prince/stay/no-dog/truth/accept-offer/keep-dog"] //done
        },
        path! {
            "/prince/stay/no-dog/truth/decline-offer",
            "You decline the Prince of Nahrin's offer.",
            "You decline the Prince of Nahrin's offer. You have embraced the life of solitude and do not want to tempt fate by engaging with other people. You leave the Prince's palace and go stay with the boys whom you've met earlier. They know that you're the a prince of Egypt so they treat you well.\n\nOne night, while you were drinking, a snake emerges from its hole to bite you. However, the Prince of Nahrin's daughter was following you, and she caught the snake, since it was imbimbed and became intoxicated so it was easy to catch. She chops the the snake into pieces. She tells you that, even though you refused her, she knows she is destined to you. Because she saved your life, you kiss her and embrace her all over her body. You eventuallly marry her.\n\nNow, the princess already had a dog that was brought to her to give her company while she stayed in the tower. You remind the princess of your fates, and that the dog might be your own end. ",
            vec!["/prince/stay/no-dog/truth/decline-offer/ask-kill-dog", "/prince/stay/no-dog/truth/decline-offer/keep-dog"] //done
        },

        path! {
            "/prince/stay/ask-for-dog/pet-dog/lie/leap",
            "You leap to reach the princess.",
            "You try to leap to reach the princess, but you have not yet recovered from the weariness of travel.",
            vec!["/prince/stay/ask-for-dog/pet-dog/lie/wait-until-healed"] //done, because other
        }, //end of path
        path! {
            "/prince/stay/ask-for-dog/pet-dog/lie/wait-until-healed",
            "You wait until you heal from the aches of travel.",
            "You wait until you heal from the aches of travel. Now you sense that you can engage with the boys trying to reach the window at the top of the tower.",
            vec!["/prince/stay/ask-for-dog/pet-dog/lie/wait-until-healed/leap"] //done
        },

        path! {
            "/prince/stay/ask-for-dog/pet-dog/truth/kill-dog",
            "You kill your dog.",
            "You kill the dog which you've reared since he was a puppy. You understand the point that the boys made, and try to escape your fate by taking precautions. You feel sad.\n\nNow, after days have passed, you still see the boys engaged in leaping every day, seemingly in order to reach the window at the top of a tower. You ask the boys why they do so. They say they're trying to reach the window of the Prince of Nahrin's daughter at the top of the tower. They say that the Prince has promised to marry off his daughter to whoever reaches her window.",
            vec!["/prince/stay/ask-for-dog/pet-dog/truth/kill-dog/leap"] //done
        },
        path! {
            "/prince/stay/ask-for-dog/pet-dog/truth/keep-dog",
            "You do not kill your dog.",
            "You are taken aback by the demand that those boys made to you. You say that you will not kill the dog that you've reared since it was a puppy. You try to change the subject. You ask what it is that those boys have been engaged in since you have arrived. They say they're trying to reach the window of the Prince of Nahrin's daughter at the top of the tower. They say that the Prince has promised to marry off his daughter to whoever reaches her window.",
            vec!["/prince/stay/ask-for-dog/pet-dog/truth/keep-dog/leap", "/prince/stay/ask-for-dog/pet-dog/truth/keep-dog/wait-until-healed"] //done
        },

        path! {
            "/prince/stay/no-dog/lie/wait-until-healed/leap",
            "You leap.",
            "You leap, successfully reaching the tower. The princess kisses you and embraces you all over your body. Then someone went in order to impart the news to her father. When the Prince of Nahrin found out about your (false) identity and that now you're going to marry his daugher, he was angered. He demanded that the messenger gets rid of you. But the daughter took hold of you and swore by God, saying, \"By Pre-Harakhti, if he is taken away from me, I shall neither eat nor drink but shall die right away.\" Then someone went to tell this to her father. And her father had you and her brought before him. You came before him, and your worth impressed the prince. He embraced you and kissed you all over your body. He said to you that now you're a son to him. Then he gave you his daughter for a wife and gave you house and fields as well as cattle and all sorts of good things.\n\nNow, after many days had elapsed, you tell your wife, \"I am committed to three fates: crocodile, snake, and even dog. I realize your dad, the Prince of Nahrin, gave you a dog since it was a puppy for you to rear and to give you company while you were in the tower, but ...\"",
            vec!["/prince/stay/no-dog/lie/wait-until-healed/leap/ask-kill-dog", "/prince/stay/no-dog/lie/wait-until-healed/leap/keep-dog"] //done
        },

        path! {
            "/prince/stay/no-dog/truth/accept-offer/ask-kill-dog",
            "You ask to kill her dog.",
            "You ask the princess to kill her dog. She screams, \"What a demand!\" She refuses to kill her dog and brushes your request as anxiety. She tries to comfort you.",
            vec!["/prince/stay/no-dog/truth/accept-offer/dog-kept"] //done
        },
        path! {
            "/prince/stay/no-dog/truth/accept-offer/keep-dog",
            "You keep the dog.",
            "You keep the dog, even though it could be your end.",
            vec!["/prince/stay/no-dog/truth/accept-offer/dog-kept"] //done
        },

        path! {
            "/prince/stay/no-dog/truth/decline-offer/ask-kill-dog",
            "You ask to kill her dog.",
            "You ask her to kill her dog. She screams, \"What a demand! I saved you from your fate once after having rejected me, and you ask me to kill the dog that I reared since it was a puppy? No.\" Since you angred the princess, the dog sensed her anger, and she fatally attacks you.",
            vec!["/end"] //end of path
        },
        path! {
            "/prince/stay/no-dog/truth/decline-offer/keep-dog",
            "You keep the dog.",
            "Even though you declined to be raised with a dog despite your loneliness fearing your fate, you feel indebted to your wife for saving you from the snake. You accept the dog's presence, even though she might be your end.\n\nDays pass, and one day on a hot day, you descend to a lake to shield yourself from the heat. However, a crocodile siezes you and takes you to where a water spirit resides, but he had left. The crocodile says to you, \"I am your fate that has been made to come in pursuit of you, but it is two whole months now that I have been fighting with the water spirit. Now see, I shall let you go. If my opponent returns to engage me to fight, come and lend me your support in order to kill the water spirit.",
            vec!["/prince/stay/no-dog/truth/decline-offer/keep-dog/help-crocodile", "/prince/stay/no-dog/truth/decline-offer/keep-dog/avoid-crocodile"] //done
        },

        path! {
            "/prince/stay/ask-for-dog/pet-dog/lie/wait-until-healed/leap",
            "You leap.",
            "You leap, successfully reaching the tower. The princess kisses you and embraces you all over your body. Then someone went in order to impart the news to her father. When the Prince of Nahrin found out about your (false) identity and that now you're going to marry his daugher, he was angered. He demanded that the messenger gets rid of you. But the daughter took hold of you and swore by God, saying, \"By Pre-Harakhti, if he is taken away from me, I shall neither eat nor drink but shall die right away.\" Then someone went to tell this to her father. And her father had you and her brought before him. You came before him, and your worth impressed the prince. He embraced you and kissed you all over your body. He said to you that now you're a son to him. Then he gave you his daughter for a wife and gave you house and fields as well as cattle and all sorts of good things.\n\nNow, after many days had elapsed, you tell your wife, \"I am committed to three fates: crocodile, snake, and dog.\" Then she says to you, \"Have the dog which follows you killed.\"",
            vec!["/prince/stay/ask-for-dog/pet-dog/lie/wait-until-healed/leap/kill-dog", "/prince/stay/ask-for-dog/pet-dog/lie/wait-until-healed/leap/keep-dog"] //done

        },

        path! {
            "/prince/stay/ask-for-dog/pet-dog/truth/kill-dog/leap",
            "You leap.",
            "You leap, successfully reaching the tower. The princess kisses you and embraces you all over your body. Then someone went in order to impart the news to her father. The king is ecstatic that he's going to marry off his daughter to a prince from Egypt. He embraces you and kisses you all over your body. He says to you that now you're a son to him. Then he gives you his daughter for a wife and gave you house and fields as well as cattle and all sorts of good things.\n\nNow, after many days had elapsed, you remind your wife of your fates, saying \"I am committed to three fates: crocodile, snake, and even dog. I realize your dad, the Prince of Nahrin, gave you a dog since it was a puppy for you to rear and to give you company while you were in the tower, but I not long ago killed my own dog which I have reared since it was a puppy, so ...\"",
            vec!["/prince/stay/ask-for-dog/pet-dog/truth/kill-dog/leap/ask-kill-dog", "/prince/stay/ask-for-dog/pet-dog/truth/kill-dog/leap/keep-dog"] //done
        },

        path! {
            "/prince/stay/ask-for-dog/pet-dog/truth/keep-dog/leap",
            "You leap to reach the princess.",
            "You try to leap to reach the princess, but you have not yet recovered from the weariness of travel.",
            vec!["/prince/stay/ask-for-dog/pet-dog/truth/keep-dog/wait-until-healed"] //done, because other
        }, //end of path
        path! {
            "/prince/stay/ask-for-dog/pet-dog/truth/keep-dog/wait-until-healed",
            "You wait until you heal from the aches of travel.",
            "You wait until you heal from the aches of travel. Now you sense that you can engage with the boys trying to reach the window at the top of the tower.",
            vec!["/prince/stay/ask-for-dog/pet-dog/truth/keep-dog/wait-until-healed/leap"] //done
        },

        path! {
            "/prince/stay/no-dog/lie/wait-until-healed/leap/ask-kill-dog",
            "You ask the princess to kill her dog.",
            "\"... you have to realize that this dog might be my end. You have to kill her.\" The princess is committed to you, and she kills her dog, \"I shall kill my dog for your own safety. This way, we have dealt with one of your fates.\"\n\nNow after some days had elapsed, you sat down and made holiday in your house. And after the end of the evening breeze you lay down upon your bed, and sleep took possession of your body. Then your wife filled one bowl with wine and another bowl with beer. Presently a snake emerged from its hole to bite you, but your wife was sitting beside you without falling asleep. The bowls were thus standing accessible to the snake, and it imbibed and became intoxicated. Then it reclined turning upside down. Thereupon your wife had it chopped to pieces with her hatchet. She then awoke you, and she told you, \"See, your god has delivered one of your fates into your hand. He will watch over you henceforth.\" Then you made an offering to Pre, praising him and extolling his power daily.\n\nDays pass, and one day on a hot day, you descend to a lake to shield yourself from the heat. However, a crocodile siezes you and takes you to where a water spirit resides, but he had left. The crocodile says to you, \"I am your fate that has been made to come in pursuit of you, but it is two whole months now that I have been fighting with the water spirit. Now see, I shall let you go. If my opponent returns to engage me to fight, come and lend me your support in order to kill the water spirit.",
            vec!["/prince/stay/no-dog/lie/wait-until-healed/leap/ask-kill-dog/help-crocodile", "/prince/stay/no-dog/lie/wait-until-healed/leap/ask-kill-dog/avoid-crocodile"] //done
        },
        path! {
             "/prince/stay/no-dog/lie/wait-until-healed/leap/keep-dog",
            "You keep the dog.",
            "\"I have come to accept my fate, and seeing that you love your dog so much, I have begun loving her too. There's no way this dog will be my fate.\" The princess was ecstatic that you did not ask her to kill her dog, since that would have made her make an impossible choice. You are both relieved.\n\nNow after some days had elapsed, you sat down and made holiday in your house. And after the end of the evening breeze you lay down upon your bed, and sleep took possession of your body. Then your wife filled one bowl with wine and another bowl with beer. Presently a snake emerged from its hole to bite you, but your wife was sitting beside you without falling asleep. The bowls were thus standing accessible to the snake, and it imbibed and became intoxicated. Then it reclined turning upside down. Thereupon your wife had it chopped to pieces with her hatchet. She then awoke you, and she told you, \"See, your god has delivered one of your fates into your hand. He will watch over you henceforth.\" Then you made an offering to Pre, praising him and extolling his power daily.\n\nDays pass, and one day on a stroll for recreation on your property, your wife's dog took on the power of speech, saying \"I am your fate.\" Thereupon you ran before her. Presently you reached the lake and descended into the water in flight before the dog. However, a crocodile siezed you and took you to where a water spirit resided, but he had left. The crocodile said to you, \"I am your fate that has been made to come in pursuit of you, but it is two whole months now that I have been fighting with the water spirit. Now see, I shall let you go. If my opponent returns to engage me to fight, come and lend me your support in order to kill the water spirit.",
            vec!["/prince/stay/no-dog/lie/wait-until-healed/leap/keep-dog/help-crocodile", "/prince/stay/no-dog/lie/wait-until-healed/leap/keep-dog/avoid-crocodile"] //done
        },

        path! {
            "/prince/stay/no-dog/truth/accept-offer/dog-kept",
            "You keep the dog.",
            "You keep the dog. You have come to accept your fate, whatever it may be.\n\nNow after some days had elapsed, you sat down and made holiday in your house. And after the end of the evening breeze you lay down upon your bed, and sleep took possession of your body. Then your wife filled one bowl with wine and another bowl with beer. Presently a snake emerged from its hole to bite you, but your wife was sitting beside you without falling asleep. The bowls were thus standing accessible to the snake, and it imbibed and became intoxicated. Then it reclined turning upside down. Thereupon your wife had it chopped to pieces with her hatchet. She then awoke you, and she told you, \"See, your god has delivered one of your fates into your hand. He will watch over you henceforth.\" Then you made an offering to Pre, praising him and extolling his power daily.\n\nDays pass, and one day on a stroll for recreation on your property, your wife's dog took on the power of speech, saying \"I am your fate.\" Thereupon you ran before her. Presently you reached the lake and descended into the water in flight before the dog. However, a crocodile siezed you and took you to where a water spirit resided, but he had left. The crocodile said to you, \"I am your fate that has been made to come in pursuit of you, but it is two whole months now that I have been fighting with the water spirit. Now see, I shall let you go. If my opponent returns to engage me to fight, come and lend me your support in order to kill the water spirit.",
            vec!["/prince/stay/no-dog/truth/accept-offer/dog-kept/help-crocodile", "/prince/stay/no-dog/truth/accept-offer/dog-kept/avoid-crocodile"] //done
        },

        path! {
            "/prince/stay/no-dog/truth/decline-offer/keep-dog/help-crocodile",
            "You agree to help the crocodile.",
            "You agree to help the crocodile. It says to you, \"In order for me to let you go, you have to help me fight the water spirit.\" Thereupon your wife's dog took on the power of speech, saying \"I will help you fight, since you seem to have accepted your fate. This shows courage.\" You three fight against the water spirit, and win. You escaped your fate, or did you?\n\nYou and your wife have trouble conceiving a child. You ask the the gods for a child, and they ordered a birth to be granted to you both. Now when she had become pregnant and had completed the months of childbearing, ...",
            vec!["/"] //done
        },
        path! {
            "/prince/stay/no-dog/truth/decline-offer/keep-dog/avoid-crocodile",
            "You try to avoid the crocodile.",
            "You try to avoid the crocodile by running from it; however, you're competing against it in its nature habitat. When it was about to catch you, your wife's dog leaps to its mouth, distracting it and giving you time to escape. As her last words, she says, \"I am your fate, i.e., I am that which concludes your destiny.\" And thus you you escaped your fate, or did you?\n\nYou and your wife have trouble conceiving a child. You ask the the gods for a child, and they ordered a birth to be granted to you both. Now when she had become pregnant and had completed the months of childbearing, ...",
            vec!["/"] //done
        },

        path! {
            "/prince/stay/ask-for-dog/pet-dog/lie/wait-until-healed/leap/kill-dog",
            "You kill your dog.",
            "Fearing your fate, you kill the dog which you've reared since he was a puppy. You feel sad while your wife tries to comfort you, but you know you did the right thing.\n\nNow after some days had elapsed and processed your loss, you sat down and made holiday in your house. And after the end of the evening breeze you lay down upon your bed, and sleep took possession of your body. Then your wife filled one bowl with wine and another bowl with beer. Presently a snake emerged from its hole to bite you, but your wife was sitting beside you without falling asleep. The bowls were thus standing accessible to the snake, and it imbibed and became intoxicated. Then it reclined turning upside down. Thereupon your wife had it chopped to pieces with her hatchet. She then awoke you, and she told you, \"See, your god has delivered one of your fates into your hand. He will watch over you henceforth.\" Then you made an offering to Pre, praising him and extolling his power daily.\n\nDays pass, and one day on a hot day, you descend to a lake to shield yourself from the heat. However, a crocodile siezes you and takes you to where a water spirit resides, but he had left. The crocodile says to you, \"I am your fate that has been made to come in pursuit of you, but it is two whole months now that I have been fighting with the water spirit. Now see, I shall let you go. If my opponent returns to engage me to fight, come and lend me your support in order to kill the water spirit.",
            vec!["/prince/stay/ask-for-dog/pet-dog/lie/wait-until-healed/leap/kill-dog/help-crocodile", "/prince/stay/ask-for-dog/pet-dog/lie/wait-until-healed/leap/kill-dog/avoid-crocodile"] //done
        },
        path! {
            "/prince/stay/ask-for-dog/pet-dog/lie/wait-until-healed/leap/keep-dog",
            "You keep your dog.",
            "You keep your dog, screaming at your wife, \"What a demand! I will not kill the dog I reared since he was a puppy.\"\n\nNow after some days had elapsed, you sat down and made holiday in your house. And after the end of the evening breeze you lay down upon your bed, and sleep took possession of your body. Then your wife filled one bowl with wine and another bowl with beer. Presently a snake emerged from its hole to bite you, but your wife was sitting beside you without falling asleep. The bowls were thus standing accessible to the snake, and it imbibed and became intoxicated. Then it reclined turning upside down. Thereupon your wife had it chopped to pieces with her hatchet. She then awoke you, and she told you, \"See, your god has delivered one of your fates into your hand. He will watch over you henceforth.\" Then you made an offering to Pre, praising him and extolling his power daily.\n\nDays pass, and one day on a stroll for recreation on your property, your dog took on the power of speech, saying \"I am your fate.\" Thereupon you ran before him. Presently you reached the lake and descended into the water in flight before the dog. However, a crocodile siezed you and took you to where a water spirit resided, but he had left. The crocodile said to you, \"I am your fate that has been made to come in pursuit of you, but it is two whole months now that I have been fighting with the water spirit. Now see, I shall let you go. If my opponent returns to engage me to fight, come and lend me your support in order to kill the water spirit.",
            vec!["/prince/stay/ask-for-dog/pet-dog/lie/wait-until-healed/leap/keep-dog/help-crocodile", "/prince/stay/ask-for-dog/pet-dog/lie/wait-until-healed/leap/keep-dog/avoid-crocodile"] //done
        },

        path! {
            "/prince/stay/ask-for-dog/pet-dog/truth/kill-dog/leap/ask-kill-dog",
            "You ask the princess to kill her dog.",
            "\"... I have to ask you to get rid of your own dog, for fear of my fate.\" She screams, \"What a demand!\" She refuses to kill her dog and brushes your request as anxiety. She tries to comfort you. You try to live with the fear of the dog being your end.\n\nNow after some days had elapsed, you sat down and made holiday in your house. And after the end of the evening breeze you lay down upon your bed, and sleep took possession of your body. Then your wife filled one bowl with wine and another bowl with beer. Presently a snake emerged from its hole to bite you, but your wife was sitting beside you without falling asleep. The bowls were thus standing accessible to the snake, and it imbibed and became intoxicated. Then it reclined turning upside down. Thereupon your wife had it chopped to pieces with her hatchet. She then awoke you, and she told you, \"See, your god has delivered one of your fates into your hand. He will watch over you henceforth.\" Then you made an offering to Pre, praising him and extolling his power daily.\n\nDays pass, and one day on a stroll for recreation on your property, your wife's dog took on the power of speech, saying \"I am your fate.\" Thereupon you ran before her. Presently you reached the lake and descended into the water in flight before the dog. However, a crocodile siezed you and took you to where a water spirit resided, but he had left. The crocodile said to you, \"I am your fate that has been made to come in pursuit of you, but it is two whole months now that I have been fighting with the water spirit. Now see, I shall let you go. If my opponent returns to engage me to fight, come and lend me your support in order to kill the water spirit.",
            vec!["/prince/stay/ask-for-dog/pet-dog/truth/kill-dog/leap/ask-kill-dog/help-crocodile", "/prince/stay/ask-for-dog/pet-dog/truth/kill-dog/leap/ask-kill-dog/avoid-crocodile"] //done
        },
        path! {
            "/prince/stay/ask-for-dog/pet-dog/truth/kill-dog/leap/keep-dog",
            "You keep the dog.",
            "\"... I have to let you know that I have grown past my fear the fear of my fate. I have also grown to love your dog. She reminds me of my own late dog.\" The princess was ecstatic that you did not ask her to kill her dog, since that would have made her make an impossible choice. You both are happy about your growth.\n\nNow after some days had elapsed, you sat down and made holiday in your house. And after the end of the evening breeze you lay down upon your bed, and sleep took possession of your body. Then your wife filled one bowl with wine and another bowl with beer. Presently a snake emerged from its hole to bite you, but your wife was sitting beside you without falling asleep. The bowls were thus standing accessible to the snake, and it imbibed and became intoxicated. Then it reclined turning upside down. Thereupon your wife had it chopped to pieces with her hatchet. She then awoke you, and she told you, \"See, your god has delivered one of your fates into your hand. He will watch over you henceforth.\" Then you made an offering to Pre, praising him and extolling his power daily.\n\nDays pass, and one day on a stroll for recreation on your property, your wife's dog took on the power of speech, saying \"I am your fate.\" Thereupon you ran before her. Presently you reached the lake and descended into the water in flight before the dog. However, a crocodile siezed you and took you to where a water spirit resided, but he had left. The crocodile said to you, \"I am your fate that has been made to come in pursuit of you, but it is two whole months now that I have been fighting with the water spirit. Now see, I shall let you go. If my opponent returns to engage me to fight, come and lend me your support in order to kill the water spirit.",
            vec!["/prince/stay/ask-for-dog/pet-dog/truth/kill-dog/leap/keep-dog/help-crocodile", "/prince/stay/ask-for-dog/pet-dog/truth/kill-dog/leap/keep-dog/avoid-crocodile"] //done
        },

        path! {
            "/prince/stay/ask-for-dog/pet-dog/truth/keep-dog/wait-until-healed/leap",
            "You leap.",
            "You leap, successfully reaching the tower. The princess kisses you and embraces you all over your body. Then someone went in order to impart the news to her father. The king is ecstatic that he's going to marry off his daughter to a prince from Egypt. He embraces you and kisses you all over your body. He says to you that now you're a son to him. Then he gives you his daughter for a wife and gave you house and fields as well as cattle and all sorts of good things.\n\nNow, one day, your wife asks you, \"If a dog is one of your fates, why don't you kill the dog which follows you?\" You tell her, \"I have considered that before, but it's the dog I have reared since it was a puppy. If he is my fate, then he is my fate. I will not kill him\" No more words are said.\n\nNow after some days had elapsed and processed your loss, you sat down and made holiday in your house. And after the end of the evening breeze you lay down upon your bed, and sleep took possession of your body. Then your wife filled one bowl with wine and another bowl with beer. Presently a snake emerged from its hole to bite you, but your wife was sitting beside you without falling asleep. The bowls were thus standing accessible to the snake, and it imbibed and became intoxicated. Then it reclined turning upside down. Thereupon your wife had it chopped to pieces with her hatchet. She then awoke you, and she told you, \"See, your god has delivered one of your fates into your hand. He will watch over you henceforth.\" Then you made an offering to Pre, praising him and extolling his power daily.\n\nDays pass, and one day on a stroll for recreation on your property, your dog took on the power of speech, saying \"I am your fate.\" Thereupon you ran before him. Presently you reached the lake and descended into the water in flight before the dog. However, a crocodile siezed you and took you to where a water spirit resided, but he had left. The crocodile said to you, \"I am your fate that has been made to come in pursuit of you, but it is two whole months now that I have been fighting with the water spirit. Now see, I shall let you go. If my opponent returns to engage me to fight, come and lend me your support in order to kill the water spirit.",
            vec!["/prince/stay/ask-for-dog/pet-dog/truth/keep-dog/wait-until-healed/leap/help-crocodile", "/prince/stay/ask-for-dog/pet-dog/truth/keep-dog/wait-until-healed/leap/avoid-crocodile"] //done
        },

        path! {
            "/prince/stay/no-dog/lie/wait-until-healed/leap/ask-kill-dog/help-crocodile",
            "You agree to help the crocodile.",
            "You agree to help the crocodile. It says to you, \"In order for me to let you go, you have to help me fight the water spirit.\" You fight with the crocodile against the water spirit, and you both triumph over it. However, your are heavily wounded from battle, since you had no help. You remain bedridden for the rest of your life, never leaving your house. At least, however, you have your wife with you.\n\nBecause of your wounds, you and your wife have trouble conceiving a child. You ask the the gods for a child, and they ordered a birth to be granted to you both. Now when she had become pregnant and had completed the months of childbearing, ...",
            vec!["/"] //done
        },
        path! {
            "/prince/stay/no-dog/lie/wait-until-healed/leap/ask-kill-dog/avoid-crocodile",
            "You try to avoid the crocodile.",
            "You try to avoid the crocodile by running from it; however, you're competing against it in its nature habitat. You lose the race and the crocodile kills you.",
            vec!["/end"] //done
        },

        path! {
            "/prince/stay/no-dog/lie/wait-until-healed/leap/keep-dog/help-crocodile",
            "You agree to help the crocodile.",
            "You agree to help the crocodile. It says to you, \"In order for me to let you go, you have to help me fight the water spirit.\" Thereupon your wife's dog took on the power of speech, saying \"I will help you fight, since you seem to have accepted your fate and began to love me, a potential enemy. This shows courage.\" You three fight against the water spirit, and win. You escaped your fate, or did you?\n\nYou and your wife have trouble conceiving a child. You ask the the gods for a child, and they ordered a birth to be granted to you both. Now when she had become pregnant and had completed the months of childbearing, ...",
            vec!["/"] //done
        },
        path! {
            "/prince/stay/no-dog/lie/wait-until-healed/leap/keep-dog/avoid-crocodile",
            "You try to avoid the crocodile.",
            "You try to avoid the crocodile by running from it; however, you're competing against it in its nature habitat. When it was about to catch you, your wife's dog leaps to its mouth, distracting it and giving you time to escape. As her last words, she says, \"I am your fate, i.e., I am that which concludes your destiny.\" And thus you you escaped your fate, or did you?\n\nYou and your wife have trouble conceiving a child. You ask the the gods for a child, and they ordered a birth to be granted to you both. Now when she had become pregnant and had completed the months of childbearing, ...",
            vec!["/"] //done
        },

        path! {
            "/prince/stay/no-dog/truth/accept-offer/dog-kept/help-crocodile",
            "You agree to help the crocodile.",
            "You agree to help the crocodile. It says to you, \"In order for me to let you go, you have to help me fight the water spirit.\" Thereupon your wife's dog took on the power of speech, saying \"I will help you fight, since you seem to have accepted your fate. This shows courage.\" You three fight against the water spirit, and win. You escaped your fate, or did you?\n\nYou and your wife have trouble conceiving a child. You ask the the gods for a child, and they ordered a birth to be granted to you both. Now when she had become pregnant and had completed the months of childbearing, ...",
            vec!["/"] //done
        },
        path! {
            "/prince/stay/no-dog/truth/accept-offer/dog-kept/avoid-crocodile",
            "You try to avoid the crocodile.",
            "You try to avoid the crocodile by running from it; however, you're competing against it in its nature habitat. When it was about to catch you, your wife's dog leaps to its mouth, distracting it and giving you time to escape. As her last words, she says, \"I am your fate, i.e., I am that which concludes your destiny.\" And thus you you escaped your fate, or did you?\n\nYou and your wife have trouble conceiving a child. You ask the the gods for a child, and they ordered a birth to be granted to you both. Now when she had become pregnant and had completed the months of childbearing, ...",
            vec!["/"] //done
        },

        path! {
            "/prince/stay/ask-for-dog/pet-dog/lie/wait-until-healed/leap/kill-dog/help-crocodile",
            "You agree to help the crocodile.",
            "You agree to help the crocodile. It says to you, \"In order for me to let you go, you have to help me fight the water spirit.\" You fight with the crocodile against the water spirit, and you both triumph over it. However, your are heavily wounded from battle, since you had no help. You remain bedridden for the rest of your life, never leaving your house. At least, however, you have your wife with you.\n\nBecause of your wounds, you and your wife have trouble conceiving a child. You ask the the gods for a child, and they ordered a birth to be granted to you both. Now when she had become pregnant and had completed the months of childbearing, ...",
            vec!["/"] //done
        },
        path! {
             "/prince/stay/ask-for-dog/pet-dog/lie/wait-until-healed/leap/kill-dog/avoid-crocodile",
            "You try to avoid the crocodile.",
            "You try to avoid the crocodile by running from it; however, you're competing against it in its nature habitat. You lose the race and the crocodile kills you.",
            vec!["/end"] //done
        },

        path! {
            "/prince/stay/ask-for-dog/pet-dog/lie/wait-until-healed/leap/keep-dog/help-crocodile",
            "You agree to help the crocodile.",
            "You agree to help the crocodile. It says to you, \"In order for me to let you go, you have to help me fight the water spirit.\" Thereupon your dog took on the power of speech, saying \"I will help you fight, since you showed courage from the moment you chose to bring me to your household. Thank you.\" You three fight against the water spirit, and win. You escaped your fate, or did you?\n\nYou and your wife have trouble conceiving a child. You ask the the gods for a child, and they ordered a birth to be granted to you both. Now when she had become pregnant and had completed the months of childbearing, ...",
            vec!["/"] //done
        },
        path! {
            "/prince/stay/ask-for-dog/pet-dog/lie/wait-until-healed/leap/keep-dog/avoid-crocodile",
            "You try to avoid the crocodile.",
            "You try to avoid the crocodile by running from it; however, you're competing against it in its nature habitat. When it was about to catch you, your dog leaps to its mouth, distracting it and giving you time to escape. As his last words, he says, \"I am your fate, i.e., I am that which concludes your destiny.\" And thus you you escaped your fate, or did you?\n\nYou and your wife have trouble conceiving a child. You ask the the gods for a child, and they ordered a birth to be granted to you both. Now when she had become pregnant and had completed the months of childbearing, ...",
            vec!["/"] //done
        },

        path! {
            "/prince/stay/ask-for-dog/pet-dog/truth/kill-dog/leap/ask-kill-dog/help-crocodile",
            "You agree to help the crocodile.",
            "You agree to help the crocodile. It says to you, \"In order for me to let you go, you have to help me fight the water spirit.\" You try to help the crocodile battling the water spirit, but you both are not winning. You speak to your wife's dog, \"Help me!\" you plead. She says to you, \"You killed me once when I was your dog. You asked your wife to kill me when I was hers. I will not help you. You do not accept your fate fully.\" Finally, the crocodile and you win, but you're extremely wounded from battle since you had no help. You remain bedridden for the rest of your life, never leaving your house. At least, however, you have your wife with you.\n\nBecause of your wounds, you and your wife have trouble conceiving a child. You ask the the gods for a child, and they ordered a birth to be granted to you both. Now when she had become pregnant and had completed the months of childbearing, ...",
            vec!["/"] //done
        },
        path! {
            "/prince/stay/ask-for-dog/pet-dog/truth/kill-dog/leap/ask-kill-dog/avoid-crocodile",
            "You try to avoid the crocodile.",
            "You try to avoid the crocodile by running from it; however, you're competing against it in its nature habitat. You lose the race and the crocodile kills you.",
            vec!["/end"] //done
        },

        path! {
            "/prince/stay/ask-for-dog/pet-dog/truth/kill-dog/leap/keep-dog/help-crocodile",
            "You agree to help the crocodile.",
            "You agree to help the crocodile. It says to you, \"In order for me to let you go, you have to help me fight the water spirit.\" Thereupon your wife's dog took on the power of speech, saying \"I will help you fight, since you seem to have accepted your fate and began to love me, a potential enemy. This shows courage.\" You three fight against the water spirit, and win. You escaped your fate, or did you?\n\nYou and your wife have trouble conceiving a child. You ask the the gods for a child, and they ordered a birth to be granted to you both. Now when she had become pregnant and had completed the months of childbearing, ...",
            vec!["/"] //done
        },
        path! {
            "/prince/stay/ask-for-dog/pet-dog/truth/kill-dog/leap/keep-dog/avoid-crocodile",
            "You try to avoid the crocodile.",
            "You try to avoid the crocodile by running from it; however, you're competing against it in its nature habitat. When it was about to catch you, your wife's dog leaps to its mouth, distracting it and giving you time to escape. As her last words, she says, \"I am your fate, i.e., I am that which concludes your destiny.\" And thus you you escaped your fate, or did you?\n\nYou and your wife have trouble conceiving a child. You ask the the gods for a child, and they ordered a birth to be granted to you both. Now when she had become pregnant and had completed the months of childbearing, ...",
            vec!["/"] //done
        },

        path! {
            "/prince/stay/ask-for-dog/pet-dog/truth/keep-dog/wait-until-healed/leap/help-crocodile",
            "You agree to help the crocodile.",
            "You agree to help the crocodile. It says to you, \"In order for me to let you go, you have to help me fight the water spirit.\" Thereupon your dog took on the power of speech, saying \"I will help you fight, since you showed courage from the moment you chose to bring me to your household. Thank you.\" You three fight against the water spirit, and win. You escaped your fate, or did you?\n\nYou and your wife have trouble conceiving a child. You ask the the gods for a child, and they ordered a birth to be granted to you both. Now when she had become pregnant and had completed the months of childbearing, ...",
            vec!["/"]
        },
        path! {
            "/prince/stay/ask-for-dog/pet-dog/truth/keep-dog/wait-until-healed/leap/avoid-crocodile",
            "You try to avoid the crocodile.",
            "You try to avoid the crocodile by running from it; however, you're competing against it in its nature habitat. When it was about to catch you, your dog leaps to its mouth, distracting it and giving you time to escape. As his last words, he says, \"I am your fate, i.e., I am that which concludes your destiny.\" And thus you you escaped your fate, or did you?\n\nYou and your wife have trouble conceiving a child. You ask the the gods for a child, and they ordered a birth to be granted to you both. Now when she had become pregnant and had completed the months of childbearing, ...",
            vec!["/"]
        },

        path! {
            "/princess",
            "Princess",
            "You are born to a Syrian Prince who locks you up. ",
            vec![]
        },
    ]
    .into();
}
