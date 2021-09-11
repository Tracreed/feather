packets! {
    Response {
        response String;
    }

    Pong {
        payload i64;
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) enum EntityStatusCodes {
 	Arrow(ArrowStatus),
 	Rabbit(RabbitStatus),
    MinecartSpawner(MinecartSpawnerStatus),
 	LivingEntity(LivingEntityStatus),
 	Snowball(SnowballStatus),
    Egg(EggStatus),
 	IronGolem(IronGolemStatus),
    EvokerFangs(EvokerFangsStatus),
    Ravager(RavagerStatus),
 	AbstractHorse(AbstractHorseStatus),
    TameableAnimal(TameableAnimalStatus),
 	Player(PlayerStatus),
 	Wolf(WolfStatus),
 	Sheep(SheepStatus),
    MinecartTNT(MinecartTNTStatus),
 	Villager(VillagerStatus),
 	Witch(WitchStatus),
 	ZombieVillager(ZombieVillagerStatus),
 	FireworkRocket(FireworkRocketStatus),
 	Animal(AnimalStatus),
 	Squid(SquidStatus),
 	Mob(MobStatus),
 	Guardian(GuardianStatus),
 	FishingHook(FishingHookStatus),
 	ArmorStand(ArmorStandStatus),
 	Dolphin(DolphinStatus),
 	Ocelot(OcelotStatus),
 	Fox(FoxStatus),
 	Entity(EntitysStatus),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) enum ArrowStatus {
    Particle = 0
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) enum RabbitStatus {
    RotatingJump = 1
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) struct MinecartSpawnerStatus;

/// Extends Entity
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) enum LivingEntityStatus {
    PlayHurt = 2,
    PlayDeath = 3,
    PlayShieldBlock = 29,
    PlayShieldBreak = 30,
    PlayHurtThorn = 31,
    PlayTotemUndying = 35,
    PlayHurtDrowning = 36,
    PlayHurtBurning = 37,
    PlayHurtSweetBerry = 44,
    PortalParticles = 46,
    /// Plays the equipment break sound (unless silent) and spawns break particles for the item in the main hand.
    PlayEquipBreakSound1 = 47,
    /// Plays the equipment break sound (unless silent) and spawns break particles for the item in the off hand.
    PlayEquipBreakSound2 = 48,
    /// Plays the equipment break sound (unless silent) and spawns break particles for the item in the head slot.
    PlayEquipBreakSound3 = 49,
    /// Plays the equipment break sound (unless silent) and spawns break particles for the item in the chest slot.
    PlayEquipBreakSound4 = 50,
    /// Plays the equipment break sound (unless silent) and spawns break particles for the item in the legs slot.
    PlayEquipBreakSound5 = 51,
    /// Plays the equipment break sound (unless silent) and spawns break particles for the item in the feet slot.
    PlayEquipBreakSound6 = 52,
    HoneyBlockFallParticles = 54,
    SwapHandItems = 55,
}

/// Inherits from LivingEntityStatus
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) enum PlayerStatus {
    /// Marks item as used (finished eating, finished drinking)
    FinishedUsing = 9,
    EnableReducedDebugScreen = 22,
    DisableReducedDebugScreen = 23,
    /// Sets Op Permission level to 0
    OpPermissionLevel0 = 24,
    OpPermissionLevel1 = 25,
    OpPermissionLevel2 = 26,
    OpPermissionLevel3 = 27,
    OpPermissionLevel4 = 28,
    CloudParticle = 43,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) enum SnowballStatus {
    DisplayParticle = 3,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) enum EggStatus {
    DisplayParticle = 3,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) enum IronGolemStatus {
    PlayAttack = 4,
    HoldOutPoppy = 11,
    PutAwayPoppy = 34,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) enum EvokerFangsStatus {
    PlayAttack = 4,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) enum RavagerStatus {
    PlayAttack = 4,
    MarkStunned = 39,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) enum AbstractHorseStatus {
    /// Taming failed.
    SmokeParticle = 6,
    /// Taming succeeded.
    HeartParticle = 7,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) enum TameableAnimalStatus {
    /// Taming failed.
    SmokeParticle = 6,
    /// Taming succeeded.
    HeartParticle = 7,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) enum WolfStatus {
    ShakingWater = 8,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) enum SheepStatus {
    EatingGrass = 10,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) enum MinecartTNTStatus {
    /// Doesn't play the ignite sound.
    Ignite = 10,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) enum VillagerStatus {
    HeartParticle = 12,
    AngryParticle = 13,
    HappyParticle = 14,
    SplashParticle = 42,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) enum WitchStatus {
    /// Spawns Between 10 - 45 WitchMagic Particles.
    WitchMagicParticle = 15
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) enum ZombieVillagerStatus {
    CureFinished = 16,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) enum FireworkRocketStatus {
    ExplodeEffect = 17,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) enum AnimalStatus {
    HeartParticle = 18,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) enum SquidStatus {
    /// Resets the squids rotation to 0 radians.
    ResetRotation = 19,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) enum MobStatus {
    ExplosionParticle = 20,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) enum GuardianStatus {
    PlayAttack = 21,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) enum FishingHookStatus {
    PlayerCaught = 31,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) enum ArmorStandStatus {
    /// Plays the hit sound and resets a hit cooldown.
    PlayHit = 32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) enum DolphinStatus {
    HappyParticle = 38,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) enum OcelotStatus {
    /// Taming failed.
    SmokeParticle = 40,
    /// Taming succeeded.
    HeartParticle = 41,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) enum FoxStatus {
    MouthItemParticle = 45,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) enum EntitysStatus {
    HoneyBlockSlide = 53,
}