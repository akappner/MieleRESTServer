#!/usr/bin/python3
# Copyright (c) 2025 Alexander Kappner.
#
# This file is part of MieleRESTServer 
# (see github).
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program. If not, see <http://www.gnu.org/licenses/>.
#
from enum import Enum, IntEnum
class DeviceId (IntEnum):
        NoDevice=0,
        WashingMachineEntryLevel=1,
        DryerEntryLevel=2,
        WashingMachineSemiPro=3,
        DryerSemiPro=4,
        WashingMachinePro=5,
        DryerPro=6,
        DishwasherEntryLevel=7,
        DishwasherSemiPro=8,
        DishwasherPro=9,
        Cooker=10,
        CookerWithMicrowave=11,
        Oven=12,
        OvenWithMicrowave=13,
        WasherDryer=24,

class ProtocolType (IntEnum):
        Unknown=0,
        Uart=1,
        MeterbusDop1=2,
        MeterbusDop2=3,
        SenseWireDop2=4,

class ProtocolVersion (IntEnum):
        Unknown=0,
        Mbus=3,
        SenseWireHdr=4

class DetergentType (IntEnum):
        NoDetergent=0,
        UltraPhase1=1,
        UltraPhase2=2,
        UltraWhite=3,
        UltraColor=4
class UserRequestOven (IntEnum):
        Nop=0,
        Start=1,
        Stop=2,
        Pause=3,
        StartDelay=8,
        DoorOpen=11,
        DoorClose=12,
        LightOn=13,
        LightOff=14,
        FactorySettingReset=15,
        SwitchOn=16,
        Next=17,
        Back=18,
        SwitchOff=19,
        ResetPinCode=20,
        KeepAlive=21,
        Step=22,
        StartRemoteUpdateInstall=23,
        ProgramStop=54,
        ProgramAbort=55,
        ProgramFinalize=56,
        ProgramSave=61,
        MotorizedFrontPanelOpen=65,
        MotorizedFrontPanelClose=66,
        HoldingBreak=68,
        HoldingStart=69,

class UserRequest (IntEnum):
        NoRequest=0,
        Start=1,
        SetInteriorLightOn=12141,
        SetInteriorLightOff=12142

class XkmRequest (IntEnum):
        NoRequest=0,
        Reset=1,
        FactorySettings=2,
        SoftApCustomer=3,
        SystemCreate=4,

class SfValueId (IntEnum):
        NONE = 0,
        OvenTimeDisplay = 3
        OvenTimePresentation = 4
        OvenTimeFormat = 5
        OvenStartScreen = 10
        OvenDisplayBrightness = 11
        OvenDisplayColorScheme = 12
        OvenVolumeSignalLevel = 14
        OvenVolumeKeyTone = 17
        OvenVolumeWelcomeMelody = 18
        OvenLighting = 21
        OvenTemperatureUnit = 22
        OvenWeightUnit = 23
        OvenSafetyKeyLock = 24
        OvenSafetyStartupLock = 25
        OvenFurnitureFrontRecognition = 26
        OvenSensorLightOnApproach = 27
        OvenSensorDisplayOnApproach = 28
        OvenSensorLowerToneOnApproach = 29
        OvenRemoteControl = 30
        OvenSupervisionFunction = 31
        OvenSupervisionDisplayStandby = 32
        OvenRemoteUpdate = 33
        OvenVoiceControl = 34

        OvenBaseLanguageCountry = 1000
        OvenQuickMicrowaveDuration = 1001
        OvenQuickMicrowavePower = 1002
        OvenPopcornDuration = 1003
        OvenKeepWarmMicrowave = 1004
        OvenKeepWarmOven = 1005
        OvenKeepWarmBoiler = 1006
        OvenAutomaticFlushing = 1007
        OvenSteamReduction = 1008
        OvenPyrolysisRequest = 1009
        OvenPanningScreen = 1010
        OvenCoolAirFollowUp = 1011
        OvenProposedTemperatures = 1012
        OvenProposedMicrowavePower = 1013
        OvenNetVoltageFrequency = 1014
        OvenProbeSensorGroup = 1015
        OvenTemperatureCalibration = 1016
        OvenCameraActivation = 1017
        OvenFastCooling = 1018
        OvenApplianceVariantId = 1019
        OvenCurrentRate = 1020
        OvenWaterHardness = 1021
        OvenCarbonateHardness = 1022
        OvenAltitude = 1023
        OvenFreshwater = 1024
        OvenPerformanceMode = 1025
        OvenSpoutAdjustment = 1026
        OvenProfileChange = 1027
        OvenExpertMode = 1028
        OvenLightOnSwitchedOn = 1029
        OvenLightOnSwitchedOff = 1030
        OvenLightOffAfter = 1031
        OvenSensorLightActivatesOff = 1032
        OvenSensorLightActivatesOn = 1033
        OvenDemoMode = 1034

        OvenAutonomousCleaningActive = 1041
        OvenAutonomousMaintenance = 1042
        OvenBeanSort = 1043
        OvenProximitySensorLightOn = 1044
        OvenTeaTimerActivation = 1045

        OvenTimer1SwitchOnTime = 1046
        OvenTimer1SwitchOffTime = 1047
        OvenTimer1SwitchOffAfter = 1048
        OvenTimer1DayOfWeek = 1049
        OvenTimer1SwitchOnActive = 1050
        OvenTimer1SwitchOffActive = 1051

        OvenTimer2SwitchOnTime = 1052
        OvenTimer2SwitchOffTime = 1053
        OvenTimer2SwitchOffAfter = 1054
        OvenTimer2DayOfWeek = 1055
        OvenTimer2SwitchOnActive = 1056
        OvenTimer2SwitchOffActive = 1057

        OvenMaintenanceTimer1SwitchOnTime = 1058
        OvenMaintenanceTimer1DayOfWeek = 1059
        OvenMaintenanceTimer1Active = 1060
        OvenMaintenanceTimer2SwitchOnTime = 1061
        OvenMaintenanceTimer2DayOfWeek = 1062
        OvenMaintenanceTimer2Active = 1063

        OvenActiveUser = 1064
        OvenFreshwaterControlEnabled = 1065
        OvenBeanPortioningEnabled = 1066
        OvenSeEnabled = 1067
        OvenSrEnabled = 1068
        OvenBooster = 1069
        OvenDisplayQuickTouch = 1070
        OvenCupHeater = 1071

        OvenMultiZoneFoodProbeAdd = 1072
        OvenMultiZoneFoodProbeSelect = 1073
        OvenPanelAutomaticMovement = 1074
        OvenSmartFoodId = 1075
        OvenAltitudeSetting = 1076

        # Proposed temperatures
        OvenThawBoiler = 1101
        OvenAutoHeating = 1102
        OvenAutoTopBottomHeat = 1103
        OvenSteamCook1 = 1104
        OvenSteamCook2 = 1105
        OvenEcoHeating = 1106
        OvenRoasting = 1107
        OvenUniversalCooking = 1108
        OvenGrill = 1109
        OvenGrillLarge = 1110
        OvenGrillSmall = 1111
        OvenHeatingPlus = 1113
        OvenIntenseBaking = 1114
        OvenCombiGrillHlp = 1115
        OvenTopBottomHeat = 1124
        OvenTopHeat = 1125
        OvenQuickHeating = 1127
        OvenConvectionGrill = 1129
        OvenBottomHeat = 1131
        OvenKeepWarm = 1132
        OvenThawDish = 1133
        OvenConvectionBake = 1136
        OvenHeatingBoiler = 1141
        OvenHeatingDish = 1142
        OvenCookFish = 1144
        OvenCookMeat = 1145
        OvenCookVegetable = 1146
        OvenClimateRoasting = 1148
        OvenClimateHeatingPlus = 1149
        OvenClimateIntenseBaking = 1150
        OvenClimateTopBottom = 1151
        OvenRotisserie = 1171
        OvenSousVideCooking = 1172
        OvenSurroundBake = 1173
        OvenEcoUniversalCooking = 1175
        OvenClimateCooking = 1176
        OvenCombiMicrowaveCooking = 1177
        OvenRotisserieLarge = 1178
        OvenRotisserieSmall = 1179
        OvenRotisserieConvection = 1180
        OvenConvectionRoast = 1181

        OvenMicrowavePower = 1219
        OvenMicrowaveRoasting = 1220
        OvenMicrowaveGrill = 1221
        OvenMicrowaveHeatingPlus = 1222
        OvenMicrowaveConvectionGrill = 1223
        OvenCombiMicrowavePower = 1277

        Global_Language=10000,
        Global_DisplayBrightness=10001,
        Global_DisplayContrast=10002,
        Global_VolumeToneSignal=10003,
        Global_VolumeToneKey=10004,
        Global_DisplayTime=10005,
        Global_TimePresentation=10006,
        Global_WaterHardness=10010,
        Global_SuperVisionDisplay=10011,

        Washer_DetergentTypeContainerOne = 12005,
        Washer_DetergentAmountContainerOne = 12006,
        Washer_DetergentTypeContainerTwo = 12007,
        Washer_DetergentAmountContainerTwo = 12008,
        Washer_SoilingDefault = 12009,
        Washer_Soiling = 12010,
        Washer_TimeDisplayFormat = 12011,
        Washer_SynchronizeTime = 12012,
        Washer_WaterCostsInteger = 12013,
        Washer_WaterCostsFractional = 12014,
        Washer_BuzzerVolume = 12015,
        Washer_KeypadTone = 12016,
        Washer_PinCodeEnable = 12017,
        Washer_PinCodeDigitOne = 12018,
        Washer_PinCodeDigitTwo = 12019,
        Washer_PinCodeDigitThree = 12020,
        Washer_TemperatureUnit = 12021,
        Washer_DisplayBrightness = 12022,
        Washer_BrightnessLightfield = 12023,
        Washer_ContractDisplay = 12024,
        Washer_DisplaySwitchOffTime = 12025,
        Washer_ApplianceSwitchOffTime = 12026,
        Washer_Memory = 12027,
        Washer_WaterInlet = 12028,
        Washer_LongerPrewashCottons = 12029,
        Washer_SoakDuration = 12030,
        Washer_GentleAction = 12031,
        Washer_TemperatureReduction = 12032,
        Washer_WaterPlus = 12033,
        Washer_WaterPlusLevel = 12034,
        Washer_MaximumRinseLevel = 12035,
        Washer_SudsCooling = 12036,
        Washer_AntiCrease = 12037,
        Washer_DelayStart = 12038,
        Washer_RemoteControl = 12039,
        Washer_LoadDosage = 12040,
        Washer_DisplayMaximumLoad = 12041,
        Washer_HeatingRating = 12043,
        Washer_MaxSpinSpeed = 12044,
        Washer_ImbalanceSensor = 12045,
        Washer_AutoDispensing = 12046,
        Washer_MainsFrequency = 12047,
        Washer_Steam = 12048,
        Washer_DrainHeight = 12049,
        Washer_IntensiveFlow = 12050,
        Washer_VolumeFlowmeterCold = 12051,
        Washer_LowWaterPressure = 12052,
        Washer_AutoLoadControl = 12053,
        Washer_ControlledEnergyConsumption = 12054,
        Washer_TemperatureIncrease = 12055,
        Washer_ReduceSpinSpeed = 12056,
        Washer_Allergy = 12057,
        Washer_ChlorineBleach = 12058,
        Washer_ResonantSpeed = 12059,
        Washer_CompressedLaundry = 12060,
        Washer_SmartGrid = 12061,
        Washer_CalibrationInitialStartup = 12063,
        Washer_WaterPath = 12064,
        Washer_MaximumSpinSpeedOffset = 12065,
        Washer_CountryVersion = 12066,
        Washer_PrewashWater = 12067,
        Washer_WaterMainWash = 12068,
        Washer_FirstRinseWater = 12069,
        Washer_FinalRinseWater = 12070,
        Washer_EmptySEnsor = 12071,
        Washer_WaterLevelCottons = 12072,
        Washer_WaterLevelMinIron = 12073,
        Washer_PrewashTimeCottons = 12074,
        Washer_WashTimeCottons = 12075,
        Washer_WashTimeMinIron = 12076,
        Washer_PreRinseCottons = 12077,
        Washer_PreRinseMinimumIron = 12078,
        Washer_PreRinseCottonsMinimumIrons = 12079,
        Washer_RinsesCottons = 12080,
        Washer_RinsesMinimumIRon = 12081,
        Washer_DisinfectionRinse = 12082,
        Washer_CapDosing = 12084,
        Washer_PreIroning = 12085,
        Washer_Hygiene = 12086,
        Washer_Standard = 12087,
        Washer_AlternatingCurrent = 12088,
        Washer_HeaterRating = 12089,
        Washer_DrainageSee = 12090,
        Washer_LyePumpCleaning = 12091,
        Washer_FaultAlarm = 12092,
        Washer_DroppedPrograms = 12093,
        Washer_LaundryRingProtection = 12094,
        Washer_LaundryRingDetection = 12095,
        Washer_CoolingWaterIntake = 12096,
        Washer_DrumOverloaded = 12097,
        Washer_TimeExtension = 12098,
        Washer_MopsWaterDrain = 12099,
        Washer_PretreatCleaningClothes = 12100,
        Washer_PinCodeSupervisorLevel = 12101,
        Washer_MieleAtHome = 12102,
        Washer_PinCodeCancelProgram = 12103,
        Washer_MopsTemperatureStdPlus = 12104,
        Washer_MopsDisinfect1 = 12105,
        Washer_MopsDisinfect2 = 12106,
        Washer_MopsMicrofiber = 12107,
        Washer_PretreatMopsRpm = 12108,
        WasherCLEANING_CLOTHS_TEMPERATURE_STD_PLUS = 12109,
        WasherCLEANING_CLOTHS_DISINFECTION = 12110,
        WasherPRETREAT_CLEANING_CLOTHS_RPM = 12111,
        WasherEARLY_WARNING_COUNTER = 12112,
        Washer_TimeOfDay = 12113,
        Washer_DisplayProgramName = 12114,
        Washer_VolumeFlowMeterWarm = 12122,
        Washer_ConsumptionData = 12123,
        Washer_LightfieldBrightnessDimmed = 12124,
        Washer_TypeOfDamper = 12125,
        Washer_CustomerService = 12126,
        Washer_ImpulsePerLiterFlowmeterCold = 12127,
        Washer_ImpulsePerLiterFlowmeterWarm = 12128,
        Washer_CapDetection = 12129,
        Washer_Greeting = 12130,
        Washer_ProgramInfo = 12131,
        Washer_DemoMode = 12132,
        Washer_ProgramEndTone = 12159,
        Washer_LanguageAccess = 12160,
        Washer_SetLanguages = 12161,
        Washer_DateFormat = 12162,
        Washer_VolumeEndTone = 12163,
        Washer_VolumeKeyTone = 12164,
        Washer_VolumeGreetingTone = 12165,
        Washer_NetworkRegistration = 12194,
        Washer_Remote = 12195,
        Washer_FactoryReset = 12196,
        Washer_Control = 12197,
        Washer_WifiFrequency = 12210,
        Washer_DeviceControl = 12215,
        Washer_StartLedBrightness = 12223,
        Washer_RemoteUpdate = 12143,
        Dryer_FactoryDefault = 16001,
        Dryer_Language = 16002,
        Dryer_ClockFormat = 16003,
        Dryer_TimeSynchronize = 16004,
        Dryer_ElectricityCostInteger = 16005,
        Dryer_ElectricityCostFractional = 16006,
        Dryer_DryingLevelCottons = 16007,
        Dryer_DryingLevelMinimumIron = 16008,
        Dryer_DryingLevelAutomatic = 16009,
        Dryer_ExtendedCoolDown = 16010,
        Dryer_CleanOutAirways = 16011,
        Dryer_BuzzerOn = 16012,
        Dryer_FinishToneVolume = 16013,
        Dryer_KeypadTone = 16014,
        Dryer_Conductivity = 16015,
        Dryer_DryingLevels = 16016,
        Dryer_TotalConsumption = 16017,
        Dryer_TotalConsumptionReset = 16018,
        Dryer_CodeActive = 16019,
        Dryer_Code = 16020,
        Dryer_DisplayBrightness = 16021,

class OperationState (IntEnum):
        Unknown=0,
        EndOfLine=1,
        Service=2,
        Settings=3,
        InitialSettings=4
        SelectProgram=5,
        RunProgram=6,
        RunDelay=7,
        RunMaintenanceProcess=8,
        VoltageBrownout=9,
        WelcomeScreen=10,
        Locked=11,
        TimeSettingScreen=12,
        DisplayOff=15,
        ColdRising=21,
        NormalRinsing=22,
        EmergencyStop=32,

class ProcessState (IntEnum):
        Unknown=0,
        NoProgram=1,
        ProgramSelected=2,
        ProgramStarted=3,
        ProgramRunning=4,
        ProgramStop=5

class ApplianceState (IntEnum):
        Unknown=0,
        Off=1,
        Synchronizing=2,
        Initializing=3,
        Normal=4,
        Demonstration=5,
        Service=6,
        Error=7,
        Check=8,
        Standby=9
        Supervisory=10,
        ShowWindow=11

class ProgramType:
        BuiltInFunction = 1
        UserDefined = 2
        Automatic = 3
        CleaningProgram = 4
        CustomerService = 5
        Helper = 6
class RemoteControl(Enum):
        Disabled = 0
        EnabledButNotPossible = 7
        Full = 15
        Unknown = 2147483647

class DeviceType (Enum):
        NoUse = 0
        WashingMachine = 1
        TumbleDryer = 2
        WashingMachineSemiPro = 3
        TumbleDryerSemiPro = 4
        WashingMachinePro = 5
        TumbleDryerPro = 6
        Dishwasher = 7
        DishwasherSemiPro = 8
        DishwasherPro = 9
        Range = 10
        RangeWithMicrowave = 11
        Oven = 12
        OvenWithMicrowave = 13
        Cooktop = 14
        SteamOven = 15
        Microwave = 16
        CoffeeMaker = 17
        Hood = 18
        Fridge = 19
        Freezer = 20
        FridgeWithFreezer = 21
        ChestFreezer = 22
        RobotVacuum = 23
        WasherDryer = 24
        WarmingDrawer = 25
        BeverageMaker = 26
class DryingStep (Enum):
        ExtraDry=0
        NormalPlus=1
        Normal=2
        SlightlyDry=3
        HandIron1=4
        HandIron2=5
        MachineIron=6
        HygieneDry=7

class Light (Enum):
        NotSupported=0
        Enabled=1
        Disabled=2
class Status (Enum):
        NoUse = 0
        Off = 1
        On = 2
        Programmed = 3
        WaitingToStart = 4
        Running = 5
        Paused = 6
        EndedSuccessfully = 7
        Failure = 8
        Abort = 9
        Idle = 10
        Rinse = 11
        Service = 12
        SuperFreeze = 13
        SuperCool = 14
        SuperHeat = 15
        Default = 144
        Lock = 145
        SuperCoolSuperFreeze = 146

class ProgramIdOven (IntEnum):
        OvenHotAirPlus=13,
        OvenIntenseBaking=14

class ProgramId (Enum): # EnumPrgId. TODO: fix overlaps with Prog IDs for non-laundry devices.
        NotSelected = 0
        Automatic = 1
        WhitesCottons = 2
        MinimumIron = 3
#        Wool = 4
        Delicates = 4
        Delicate = 5
        HotAir = 6
        ColdAir = 7
#        Express = 8
        Wool = 8
        Cotton = 9
        Gentle = 10
        CottonHygiene = 11
        DrainSpin = 21
        Shirts = 23
        Waterproofing = 27
#        Cottons40Celsius = 27
        Cottons25Celsius = 28
        MinimumIron25Celsius = 29
        SyntheticBedding = 30
        NaturalBedding = 31
        Microfiber = 32
        WetcareIntensive = 33
        WetcareSensitive = 34
        WetcareSilk = 35
        LargeItems = 36
        Reactivate = 37
        Smoothing = 38
        CottonsWhiteHygiene = 39
        Maintenance = 48
        ProgramFortyCelsius = 59
        ExpressTwentyMinutes = 122
        DarkGarmentsDenim = 123
        EuropeanUnionEcoMode = 190

class ProgramPhase(Enum):
        NotUsed = 0
        Progress = 1
        BatteryCharging = 2
        WashingMachineIdle = 256
        WashingMachinePreWash = 257
        WashingMachineSoak = 258
        WashingMachinePreRinse = 259
        WashingMachineWashing = 260
        WashingMachineRinse = 261
        WashingMachineRinseHold = 262
        WashingMachineClean = 263
        WashingMachineCooldown = 264
        WashingMachineDrain = 265
        WashingMachineSpin = 266
        WashingMachineAntiCrease = 267
        WashingMachineFinished = 268
        WashingMachineVenting = 269
        WashingMachineStarch = 270
        WashingMachineMoisting = 271
        WashingMachineRemoisting = 272
        WashingMachineHygiene = 279
        WashingMachineDrying = 280
        WashingMachineDisinfection = 285
        WashingMachineSteamSmoothing = 295
        TumbleDryerIdle = 512
        TumbleDryerProgramStart = 513
        TumbleDryerDrying = 514
        TumbleDryerMachineIron = 515
        TumbleDryerHandIron2 = 516
        TumbleDryerNormal = 517
        TumbleDryerNormalPlus = 518
        TumbleDryerComfortCooldown = 519
        TumbleDryerHandIron1 = 520
        TumbleDryerAntiCreaseFinish = 521
        TumbleDryerFinished = 522
        TumbleDryerExtraDry = 523
        TumbleDryerHandIronWithoutDrop = 524
        TumbleDryerHygieneDrying = 525
        TumbleDryerWetting = 526
        TumbleDryerSpin = 527
        TumbleDryerFanActive = 528
        TumbleDryerHotAir = 529
        TumbleDryerSteamSmooth = 530
        TumbleDryerCooling = 531
        TumbleDryerFluff = 532
        TumbleDryerRinse = 533
        TumbleDryerSmooth = 534
        TumbleDryerUnknown1 = 535
        TumbleDryerRemoteStart = 536
        TumbleDryerDelayedRun = 537
        TumbleDryerSlightlyDry = 538
        TumbleDryerSafetyCooldown = 539
        DishwasherNoProgram = 1792
        DishwasherRegenerate = 1793
        DishwasherPreRinse = 1794
        DishwasherClean = 1795
        DishwasherRinse = 1796
        DishwasherRinseInterim = 1797
        DishwasherRinseClear = 1798
        DishwasherDrying = 1799
        DishwasherFinished = 1800
        DishwasherPreRinse2 = 1801
        OvenCooldown = 3072
        OvenHeating = 3073
        OvenTempHold = 3074
        OvenDoorOpen = 3075
        OvenPyrolyze = 3076
        OvenMicrowave = 3077
        OvenProgramDone = 3078
        OvenLight = 3079
        OvenSearing = 3080
        OvenRoasting = 3081
        OvenDefrost = 3082
        OvenCooldown2 = 3083
        OvenEnergySave = 3084
        OvenHoldWarm = 3094
        OvenDescale = 3098
        OvenPreheat = 3099
