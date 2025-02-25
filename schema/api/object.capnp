@0xfb858fb992d10849;

using import "../aliases/color.capnp".Color;
using import "../aliases/enum.capnp".EnumValue;
using import "../aliases/flag.capnp".FlagValue;
using import "../aliases/time.capnp".Duration;
using import "../option.capnp".Option;
using import "aliases/count.capnp".Count;
using import "aliases/geometry.capnp".Modifier;
using import "aliases/geometry.capnp".Rate;
using import "aliases/geometry.capnp".Rotation;
using import "aliases/geometry.capnp".Scale;
using import "aliases/geometry.capnp".Unit;
using import "aliases/id.capnp".AnimationId;
using import "aliases/id.capnp".BlockId;
using import "aliases/id.capnp".CoinId;
using import "aliases/id.capnp".ColorChannelId;
using import "aliases/id.capnp".GroupId;
using import "aliases/id.capnp".ItemId;
using import "aliases/id.capnp".ObjectId;
using import "aliases/id.capnp".LinkId;
using import "aliases/layer.capnp".EditorLayer;
using import "aliases/layer.capnp".ZLayer;
using import "aliases/opacity.capnp".Opacity;
using import "aliases/opacity.capnp".OptionOpacity;
using import "aliases/order.capnp".ZOrder;
using import "aliases/physics.capnp".Speed;
using import "aliases/physics.capnp".Strength;
using import "hsv.capnp".Hsv;

struct ObjectTransform {
    hFlipped @0 :Bool;
    vFlipped @1 :Bool;
    rotation @2 :Rotation;
    scale @3 :Scale;
}

struct ObjectVisual {
    doNotFade @0 :Bool;
    doNotEnter @1 :Bool;
    disableGlow @2 :Bool;
}

struct ObjectZ {
    layer @0 :ZLayer;
    order @1 :ZOrder;
}

struct ObjectEditorLayers {
    base @0 :EditorLayer;
    additional @1 :EditorLayer;
}

struct ObjectColors {
    baseChannelId @0 :ColorChannelId;
    detailChannelId @1 :ColorChannelId;
    baseHsv @2 :Hsv;
    detailHsv @3 :Hsv;
}

struct ObjectSpecial {
    checked @0 :Bool;
    unknown @1 :Bool;
    highDetail @2 :Bool;
    groupParent @3 :Bool;
}

struct StartPositionOptions {
    gameMode @0 :EnumValue;
    miniMode @1 :Bool;
    speed @2 :EnumValue;
    dualMode @3 :Bool;
    flipGravity @4 :Bool;
}

struct SecretCoinOptions {
    coinId @0 :CoinId;
}

struct RotatingObjectOptions {
    rotationSpeed @0 :Speed;
    disableRotation @1 :Bool;
}

struct TextOptions {
    content @0 :Text;
}

struct TeleportOptions {
    portalOffset @0 :Unit;
    smooth @1 :Bool;
}

struct PulsatingObjectOptions {
    animationSpeed @0 :Speed;
    randomizeStart @1 :Bool;
}

struct CollisionBlockOptions {
    blockId @0 :BlockId;
    dynamic @1 :Bool;
}

struct TriggerOrbOptions {
    targetGroupId @0 :GroupId;
    activateGroup @1 :Bool;
}

struct OrbOptions {
    multiActivate @0 :Bool;
    union {
        simple @1 :Void;
        trigger @2 :TriggerOrbOptions;
    }
}

struct ItemCounterOptions {
    itemId @0 :ItemId;
}

struct ToggleItemOptions {
    targetGroupId @0 :GroupId;
    activateGroup @1 :Bool;
}

struct PickupItemOptions {
    itemId @0 :ItemId;
    subtractCount @1 :Bool;
}

struct ItemOptions {
    union {
        toggle @0 :ToggleItemOptions;
        pickup @1 :PickupItemOptions;
    }
}

struct PlayerColorTriggerOptions {
    playerColor @0 :EnumValue;
    opacity @1 :Opacity;
}

struct NormalColorTriggerOptions {
    color @0 :Color;
    opacity @1 :Opacity;
}

struct CopiedColorTriggerOptions {
    copiedColorChannelId @0 :ColorChannelId;
    copiedHsv @1 :Hsv;
    opacity @2 :OptionOpacity;
}

struct ColorTriggerOptions {
    targetColorChannelId @0 :ColorChannelId;
    blending @1 :Bool;
    duration @2 :Duration;
    union {
        player @3 :PlayerColorTriggerOptions;
        normal @4 :NormalColorTriggerOptions;
        copied @5 :CopiedColorTriggerOptions;
    }
}

struct PulseColorTriggerOptions {
    color @0 :Color;
}

struct PulseHSVTriggerOptions {
    copiedColorChannelId @0 :ColorChannelId;
    copiedHsv @1 :Hsv;
}

struct PulseColorChannelTriggerOptions {
    targetColorChannelId @0 :ColorChannelId;
    union {
        color @1 :PulseColorTriggerOptions;
        hsv @2 :PulseHSVTriggerOptions;
    }
}

struct PulseGroupTriggerOptions {
    targetGroupId @0 :GroupId;
    pulseType @1 :FlagValue;
    union {
        color @2 :PulseColorTriggerOptions;
        hsv @3 :PulseHSVTriggerOptions;
    }
}

struct PulseTriggerOptions {
    fadeIn @0 :Duration;
    hold @1 :Duration;
    fadeOut @2 :Duration;
    exclusive @3 :Bool;
    union {
        colorChannel @4 :PulseColorChannelTriggerOptions;
        group @5 :PulseGroupTriggerOptions;
    }
}

struct AlphaTriggerOptions {
    targetGroupId @0 :GroupId;
    duration @1 :Duration;
    opacity @2 :Opacity;
}

struct NormalMoveTriggerOptions {
    xOffset @0 :Unit;
    yOffset @1 :Unit;
    lockedToPlayer @2 :FlagValue;
}

struct TargetMoveTriggerOptions {
    additionalGroupId @0 :GroupId;
    targetType @1 :FlagValue;
}

struct MoveTriggerOptions {
    targetGroupId @0 :GroupId;
    easing @1 :EnumValue;
    easingRate @2 :Rate;
    duration @3 :Duration;
    union {
        normal @4 :NormalMoveTriggerOptions;
        target @5 :TargetMoveTriggerOptions;
    }
}

struct SpawnTriggerOptions {
    targetGroupId @0 :GroupId;
    delay @1 :Duration;
    editorDisable @2 :Bool;
}

struct StopTriggerOptions {
    targetGroupId @0 :GroupId;
}

struct ToggleTriggerOptions {
    targetGroupId @0 :GroupId;
    activateGroup @1 :Bool;
}

struct RotateTriggerOptions {
    targetGroupId @0 :GroupId;
    additionalGroupId @1 :GroupId;
    duration @2 :Duration;
    easing @3 :EnumValue;
    easingRate @4 :Rate;
    targetRotation @5 :Rotation;
    rotationLocked @6 :Bool;
}

struct FollowTriggerOptions {
    targetGroupId @0 :GroupId;
    additionalGroupId @1 :GroupId;
    duration @2 :Duration;
    easing @3 :EnumValue;
    easingRate @4 :Rate;
    xModifier @5 :Modifier;
    yModifier @6 :Modifier;
}

struct ShakeTriggerOptions {
    duration @0 :Duration;
    strength @1 :Strength;
    interval @2 :Duration;
}

struct AnimateTriggerOptions {
    targetGroupId @0 :GroupId;
    animationId @1 :AnimationId;
}

struct TouchTriggerOptions {
    targetGroupId @0 :GroupId;
    holdMode @1 :Bool;
    dualMode @2 :Bool;
    toggleType @3 :EnumValue;
}

struct CountTriggerOptions {
    itemId @0 :ItemId;
    count @1 :Count;
    activateGroup @2 :Bool;
    multiActivate @3 :Bool;
}

struct InstantCountTriggerOptions {
    itemId @0 :ItemId;
    count @1 :Count;
    activateGroup @2 :Bool;
    comparison @3 :EnumValue;
}

struct PickupTriggerOptions {
    itemId @0 :ItemId;
    count @1 :Count;
}

struct FollowPlayerYTriggerOptions {
    targetGroupId @0 :GroupId;
    duration @1 :Duration;
    delay @2 :Duration;
    speed @3 :Speed;
    maxSpeed @4 :Speed;
    offset @5 :Unit;
}

struct OnDeathTriggerOptions {
    targetGroupId @0 :GroupId;
    activateGroup @1 :Bool;
}

struct CollisionTriggerOptions {
    targetGroupId @0 :GroupId;
    activateGroup @1 :Bool;
    blockAId @2 :BlockId;
    blockBId @3 :BlockId;
    triggerOnExit @4 :Bool;
}

struct TriggerOptions {
    touchTriggered @0 :Bool;
    spawnTriggered @1 :Bool;
    multiTrigger @2 :Bool;
    union {
        color @3 :ColorTriggerOptions;
        pulse @4 :PulseTriggerOptions;
        alpha @5 :AlphaTriggerOptions;
        move @6 :MoveTriggerOptions;
        spawn @7 :SpawnTriggerOptions;
        stop @8 :StopTriggerOptions;
        toggle @9 :ToggleTriggerOptions;
        rotate @10 :RotateTriggerOptions;
        follow @11 :FollowTriggerOptions;
        shake @12 :ShakeTriggerOptions;
        animate @13 :AnimateTriggerOptions;
        touch @14 :TouchTriggerOptions;
        count @15 :CountTriggerOptions;
        instantCount @16 :InstantCountTriggerOptions;
        pickup @17 :PickupTriggerOptions;
        followPlayerY @18 :FollowPlayerYTriggerOptions;
        onDeath @19 :OnDeathTriggerOptions;
        collision @20 :CollisionTriggerOptions;
    }
}

struct AnyObject {
    id @0 :ObjectId;
    x @1 :Unit;
    y @2 :Unit;
    transform @3 :Option(ObjectTransform);
    visual @4 :Option(ObjectVisual);
    z @5 :Option(ObjectZ);
    editorLayers @6 :Option(ObjectEditorLayers);
    colors @7 :Option(ObjectColors);
    special @8 :Option(ObjectSpecial);
    groupIds @9 :List(GroupId);
    union {
        simple @10 :Void;
        startPosition @11 :StartPositionOptions;
        secretCoin @12 :SecretCoinOptions;
        rotatingObject @13 :RotatingObjectOptions;
        text @14 :TextOptions;
        teleport @15 :TeleportOptions;
        pulsatingObject @16 :PulsatingObjectOptions;
        collisionBlock @17 :CollisionBlockOptions;
        orb @18 :OrbOptions;
        itemCounter @19 :ItemCounterOptions;
        item @20 :ItemOptions;
        trigger @21 :TriggerOptions;
    }
}
