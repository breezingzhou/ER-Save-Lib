use std::{collections::HashMap, sync::LazyLock};

pub static ARTS_NAME_DLC01: LazyLock<HashMap<u32, &'static str>> = LazyLock::new(|| {
    HashMap::from([
		(2000,"Dryleaf Whirlwind"),
		(2001,"Aspects of the Crucible: Wings"),
		(4000,"Spinning Gravity Thrust"),
		(4001,""),
		(4002,""),
		(4003,""),
		(4004,""),
		(4005,""),
		(4006,""),
		(4007,""),
		(4008,""),
		(4009,""),
		(4010,"Palm Blast"),
		(4011,""),
		(4012,""),
		(4013,""),
		(4014,""),
		(4015,""),
		(4016,""),
		(4017,""),
		(4018,""),
		(4019,""),
		(4020,"Piercing Throw"),
		(4021,""),
		(4022,""),
		(4023,""),
		(4024,""),
		(4025,""),
		(4026,""),
		(4027,""),
		(4028,""),
		(4029,""),
		(4030,"Scattershot Throw"),
		(4031,""),
		(4032,""),
		(4033,""),
		(4034,""),
		(4035,""),
		(4036,""),
		(4037,""),
		(4038,""),
		(4039,""),
		(4040,"Wall of Sparks"),
		(4041,""),
		(4042,""),
		(4043,""),
		(4044,""),
		(4045,""),
		(4046,""),
		(4047,""),
		(4048,""),
		(4049,""),
		(4050,"Rolling Sparks"),
		(4051,""),
		(4052,""),
		(4053,""),
		(4054,""),
		(4055,""),
		(4056,""),
		(4057,""),
		(4058,""),
		(4059,""),
		(4060,"Raging Beast"),
		(4061,""),
		(4062,""),
		(4063,""),
		(4064,""),
		(4065,""),
		(4066,""),
		(4067,""),
		(4068,""),
		(4069,""),
		(4070,"Savage Claws"),
		(4071,""),
		(4072,""),
		(4073,""),
		(4074,""),
		(4075,""),
		(4076,""),
		(4077,""),
		(4078,""),
		(4079,""),
		(4080,"Red Bear Hunt"),
		(4081,""),
		(4082,""),
		(4083,""),
		(4084,""),
		(4085,""),
		(4086,""),
		(4087,""),
		(4088,""),
		(4089,""),
		(4090,"Blind Spot"),
		(4091,""),
		(4092,""),
		(4093,""),
		(4094,""),
		(4095,""),
		(4096,""),
		(4097,""),
		(4098,""),
		(4099,""),
		(4100,"Swift Slash"),
		(4101,""),
		(4102,""),
		(4103,""),
		(4104,""),
		(4105,""),
		(4106,""),
		(4107,""),
		(4108,""),
		(4109,""),
		(4110,"Overhead Stance"),
		(4111,""),
		(4112,""),
		(4113,""),
		(4114,""),
		(4115,""),
		(4116,""),
		(4117,""),
		(4118,""),
		(4119,""),
		(4120,"Wing Stance"),
		(4121,""),
		(4122,""),
		(4123,""),
		(4124,""),
		(4125,""),
		(4126,""),
		(4127,""),
		(4128,""),
		(4129,""),
		(4130,"Blinkbolt"),
		(4131,""),
		(4132,""),
		(4133,""),
		(4134,""),
		(4135,""),
		(4136,""),
		(4137,""),
		(4138,""),
		(4139,""),
		(4140,"Flame Skewer"),
		(4141,""),
		(4142,""),
		(4143,""),
		(4144,""),
		(4145,""),
		(4146,""),
		(4147,""),
		(4148,""),
		(4149,""),
		(4150,"Savage Lion's Claw"),
		(4151,""),
		(4152,""),
		(4153,""),
		(4154,""),
		(4155,""),
		(4156,""),
		(4157,""),
		(4158,""),
		(4159,""),
		(4160,"Divine Beast Frost Stomp"),
		(4161,""),
		(4162,""),
		(4163,""),
		(4164,""),
		(4165,""),
		(4166,""),
		(4167,""),
		(4168,""),
		(4169,""),
		(4170,"Flame Spear"),
		(4171,""),
		(4172,""),
		(4173,""),
		(4174,""),
		(4175,""),
		(4176,""),
		(4177,""),
		(4178,""),
		(4179,""),
		(4180,"Carian Sovereignty"),
		(4181,""),
		(4182,""),
		(4183,""),
		(4184,""),
		(4185,""),
		(4186,""),
		(4187,""),
		(4188,""),
		(4189,""),
		(4190,"Shriek of Sorrow"),
		(4191,""),
		(4192,""),
		(4193,""),
		(4194,""),
		(4195,""),
		(4196,""),
		(4197,""),
		(4198,""),
		(4199,""),
		(4200,"Shield Strike"),
		(4201,""),
		(4202,""),
		(4203,""),
		(4204,""),
		(4205,""),
		(4206,""),
		(4207,""),
		(4208,""),
		(4209,""),
		(4210,"Igon's Drake Hunt"),
		(4211,""),
		(4212,""),
		(4213,""),
		(4214,""),
		(4215,""),
		(4216,""),
		(4217,""),
		(4218,""),
		(4219,""),
		(4220,"Ghostflame Call"),
		(5000,"Dragonwound Slash"),
		(5001,""),
		(5002,""),
		(5003,""),
		(5004,""),
		(5005,""),
		(5006,""),
		(5007,""),
		(5008,""),
		(5009,""),
		(5010,"Needle Piercer"),
		(5011,""),
		(5012,""),
		(5013,""),
		(5014,""),
		(5015,""),
		(5016,""),
		(5017,""),
		(5018,""),
		(5019,""),
		(5020,"Light"),
		(5021,""),
		(5022,""),
		(5023,""),
		(5024,""),
		(5025,""),
		(5026,""),
		(5027,""),
		(5028,""),
		(5029,""),
		(5030,"Darkness"),
		(5031,""),
		(5032,""),
		(5033,""),
		(5034,""),
		(5035,""),
		(5036,""),
		(5037,""),
		(5038,""),
		(5039,""),
		(5040,"Onze's Line of Stars"),
		(5041,""),
		(5042,""),
		(5043,""),
		(5044,""),
		(5045,""),
		(5046,""),
		(5047,""),
		(5048,""),
		(5049,""),
		(5050,"The Poison Flower Blooms Twice"),
		(5080,"Spinning Guillotine"),
		(5081,""),
		(5082,""),
		(5083,""),
		(5084,""),
		(5085,""),
		(5086,""),
		(5087,""),
		(5088,""),
		(5089,""),
		(5090,"Unending Dance"),
		(5091,""),
		(5092,""),
		(5093,""),
		(5094,""),
		(5095,""),
		(5096,""),
		(5097,""),
		(5098,""),
		(5099,""),
		(5100,"Revenger's Blade"),
		(5101,""),
		(5102,""),
		(5103,""),
		(5104,""),
		(5105,""),
		(5106,""),
		(5107,""),
		(5108,""),
		(5109,""),
		(5110,"Mists of Eternal Sleep"),
		(5111,""),
		(5112,""),
		(5113,""),
		(5114,""),
		(5115,""),
		(5116,""),
		(5117,""),
		(5118,""),
		(5119,""),
		(5120,"Dynastic Sickleplay"),
		(5121,""),
		(5122,""),
		(5123,""),
		(5124,""),
		(5125,""),
		(5126,""),
		(5127,""),
		(5128,""),
		(5129,""),
		(5130,"Blinkbolt: Twinaxe"),
		(5131,""),
		(5132,""),
		(5133,""),
		(5134,""),
		(5135,""),
		(5136,""),
		(5137,""),
		(5138,""),
		(5139,""),
		(5140,"Blinkbolt: Long-hafted Axe"),
		(5141,""),
		(5142,""),
		(5143,""),
		(5144,""),
		(5145,""),
		(5146,""),
		(5147,""),
		(5148,""),
		(5149,""),
		(5150,"Promised Consort"),
		(5151,""),
		(5152,""),
		(5153,""),
		(5154,""),
		(5155,""),
		(5156,""),
		(5157,""),
		(5158,""),
		(5159,""),
		(5160,"Shadow Sunflower Headbutt"),
		(5161,""),
		(5162,""),
		(5163,""),
		(5164,""),
		(5165,""),
		(5166,""),
		(5167,""),
		(5168,""),
		(5169,""),
		(5170,"Moon-and-Fire Stance"),
		(5171,""),
		(5172,""),
		(5173,""),
		(5174,""),
		(5175,""),
		(5176,""),
		(5177,""),
		(5178,""),
		(5179,""),
		(5180,"Devonia's Vortex"),
		(5181,""),
		(5182,""),
		(5183,""),
		(5184,""),
		(5185,""),
		(5186,""),
		(5187,""),
		(5188,""),
		(5189,""),
		(5190,"Messmer's Assault"),
		(5210,"Sleep Evermore"),
		(5211,""),
		(5212,""),
		(5213,""),
		(5214,""),
		(5215,""),
		(5216,""),
		(5217,""),
		(5218,""),
		(5219,""),
		(5220,"Golden Crux"),
		(5221,""),
		(5222,""),
		(5223,""),
		(5224,""),
		(5225,""),
		(5226,""),
		(5227,""),
		(5228,""),
		(5229,""),
		(5230,"Moore's Charge"),
		(5231,""),
		(5232,""),
		(5233,""),
		(5234,""),
		(5235,""),
		(5236,""),
		(5237,""),
		(5238,""),
		(5239,""),
		(5240,"White Light Charge"),
		(5241,""),
		(5242,""),
		(5243,""),
		(5244,""),
		(5245,""),
		(5246,""),
		(5247,""),
		(5248,""),
		(5249,""),
		(5250,"Rancor Slash"),
		(5251,""),
		(5252,""),
		(5253,""),
		(5254,""),
		(5255,""),
		(5256,""),
		(5257,""),
		(5258,""),
		(5259,""),
		(5260,"Witching Hour Slash"),
		(5261,""),
		(5262,""),
		(5263,""),
		(5264,""),
		(5265,""),
		(5266,""),
		(5267,""),
		(5268,""),
		(5269,""),
		(5270,"Euporia Vortex"),
		(5271,""),
		(5272,""),
		(5273,""),
		(5274,""),
		(5275,""),
		(5276,""),
		(5277,""),
		(5278,""),
		(5279,""),
		(5280,"Smithing Art Spears"),
		(5281,""),
		(5282,""),
		(5283,""),
		(5284,""),
		(5285,""),
		(5286,""),
		(5287,""),
		(5288,""),
		(5289,""),
		(5290,"Rancor Slash"),
		(5291,""),
		(5292,""),
		(5293,""),
		(5294,""),
		(5295,""),
		(5296,""),
		(5297,""),
		(5298,""),
		(5299,""),
		(5300,"Romina's Purification"),
		(5301,""),
		(5302,""),
		(5303,""),
		(5304,""),
		(5305,""),
		(5306,""),
		(5307,""),
		(5308,""),
		(5309,""),
		(5310,"Poison Spear-Hand Strike"),
		(5311,""),
		(5312,""),
		(5313,""),
		(5314,""),
		(5315,""),
		(5316,""),
		(5317,""),
		(5318,""),
		(5319,""),
		(5320,"Madding Spear-Hand Strike"),
		(5321,""),
		(5322,""),
		(5323,""),
		(5324,""),
		(5325,""),
		(5326,""),
		(5327,""),
		(5328,""),
		(5329,""),
		(5330,"Feeble Lord's Frenzied Flame"),
		(5331,""),
		(5332,""),
		(5333,""),
		(5334,""),
		(5335,""),
		(5336,""),
		(5337,""),
		(5338,""),
		(5339,""),
		(5340,"Repeating Fire"),
		(5341,""),
		(5342,""),
		(5343,""),
		(5344,""),
		(5345,""),
		(5346,""),
		(5347,""),
		(5348,""),
		(5349,""),
		(5350,"Deadly Dance"),
		(5351,""),
		(5352,""),
		(5353,""),
		(5354,""),
		(5355,""),
		(5356,""),
		(5357,""),
		(5358,""),
		(5359,""),
		(5360,"Fan Shot"),
		(5361,""),
		(5362,""),
		(5363,""),
		(5364,""),
		(5365,""),
		(5366,""),
		(5367,""),
		(5368,""),
		(5369,""),
		(5370,"Discus Hurl"),
		(5371,""),
		(5372,""),
		(5373,""),
		(5374,""),
		(5375,""),
		(5376,""),
		(5377,""),
		(5378,""),
		(5379,""),
		(5380,"Flower Dragonbolt"),
		(5381,""),
		(5382,""),
		(5383,""),
		(5384,""),
		(5385,""),
		(5386,""),
		(5387,""),
		(5388,""),
		(5389,""),
		(5390,"Kowtower's Resentment"),
		(5420,"Painful Strike"),
		(5421,""),
		(5422,""),
		(5423,""),
		(5424,""),
		(5425,""),
		(5426,""),
		(5427,""),
		(5428,""),
		(5429,""),
		(5430,"Solitary Moon Slash"),
		(5431,""),
		(5432,""),
		(5433,""),
		(5434,""),
		(5435,""),
		(5436,""),
		(5437,""),
		(5438,""),
		(5439,""),
		(5440,"Revenge of the Night"),
		(5441,""),
		(5442,""),
		(5443,""),
		(5444,""),
		(5445,""),
		(5446,""),
		(5447,""),
		(5448,""),
		(5449,""),
		(5450,"Weed Cutter"),
		(5451,""),
		(5452,""),
		(5453,""),
		(5454,""),
		(5455,""),
		(5456,""),
		(5457,""),
		(5458,""),
		(5459,""),
		(5460,"Blindfold of Happiness"),
		(5461,""),
		(5462,""),
		(5463,""),
		(5464,""),
		(5465,""),
		(5466,""),
		(5467,""),
		(5468,""),
		(5469,""),
		(5470,"Jori's Inquisition"),
		(5471,""),
		(5472,""),
		(5473,""),
		(5474,""),
		(5475,""),
		(5476,""),
		(5477,""),
		(5478,""),
		(5479,""),
		(5480,"Roaring Bash"),
		(5481,""),
		(5482,""),
		(5483,""),
		(5484,""),
		(5485,""),
		(5486,""),
		(5487,""),
		(5488,""),
		(5489,""),
		(5490,"Deadly Poison Spray"),
		(5491,""),
		(5492,""),
		(5493,""),
		(5494,""),
		(5495,""),
		(5496,""),
		(5497,""),
		(5498,""),
		(5499,""),
		(5500,"Flare, O Serpent"),
		(5501,""),
		(5502,""),
		(5503,""),
		(5504,""),
		(5505,""),
		(5506,""),
		(5507,""),
		(5508,""),
		(5509,""),
		(5510,"Scattershot (Claws)"),
		(5511,""),
		(5512,""),
		(5513,""),
		(5514,""),
		(5515,""),
		(5516,""),
		(5517,""),
		(5518,""),
		(5519,""),
		(5520,"Hone Blade"),
		(5521,""),
		(5522,""),
		(5523,""),
		(5524,""),
		(5525,""),
		(5526,""),
		(5527,""),
		(5528,""),
		(5529,""),
		(5530,"Horn Calling"),
		(5531,""),
		(5532,""),
		(5533,""),
		(5534,""),
		(5535,""),
		(5536,""),
		(5537,""),
		(5538,""),
		(5539,""),
		(5540,"Horn Calling: Storm"),
		(5541,""),
		(5542,""),
		(5543,""),
		(5544,""),
		(5545,""),
		(5546,""),
		(5547,""),
		(5548,""),
		(5549,""),
		(5550,"Tremendous Phalanx"),
		(5551,""),
		(5552,""),
		(5553,""),
		(5554,""),
		(5555,""),
		(5556,""),
		(5557,""),
		(5558,""),
		(5559,""),
		(5560,"Bloodfiends' Bloodboon"),
		(5561,""),
		(5562,""),
		(5563,""),
		(5564,""),
		(5565,""),
		(5566,""),
		(5567,""),
		(5568,""),
		(5569,""),
		(5570,"Dragonform Flame"),
		(5571,""),
		(5572,""),
		(5573,""),
		(5574,""),
		(5575,""),
		(5576,""),
		(5577,""),
		(5578,""),
		(5579,""),
		(5580,"Lightspeed Slash"),
		(5600,"Rancor Shot"),
])});