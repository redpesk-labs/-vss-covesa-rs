
WARNING: this is a work in progress, expect continuous changes until summer.
----------------------------------------------------------------------------

testing:
--------
 - cargo build
 - vss-parser example/etc/vspec/test_allowed/test.vspec

Dependencies:
-------------
get official spec from: https://github.com/COVESA/vehicle_signal_specification
 - vss-parser vehicle_signal_specification/spec/VehicleSignalSpecification.vspec

VssParser:
----------
 * provision Rust object for [Branch,Sensor,Attributes]
 * keep track of original vspec (filename + line number)

```
-- vpath: Vehicle.Powertrain.FuelSystem.SupportedFuel  (FuelSystem.vspec:21)
   type: attribute
   description: RON 95 is sometimes referred to as Super, RON 98 as Super Plus.
   arraysize: 0
   datatype: string
   allowed:
     E5_95,
     XTL,
   unit: Unset

-- vpath: Vehicle.Powertrain.FuelSystem.HybridType  (FuelSystem.vspec:31)
   type: attribute
   description: Defines the hybrid type of the vehicle.
   datatype: string
   default: UNKNOWN
   allowed:
     UNKNOWN,
     NOT_APPLICABLE,
     STOP_START,
     BELT_ISG,
     CIMG,
     PHEV,
   unit: Unset

-- vpath: Vehicle.Powertrain.FuelSystem.TankCapacity  (FuelSystem.vspec:38)
   type: attribute
   description: Capacity of the fuel tank in liters.
   datatype: float
   unit: l
```


Fulup VSS notes:
----------------
VSS documentation broken link:  (il faudrait passer un crower sur les markdown)
 page: https://covesa.github.io/vehicle_signal_specification/rule_set/data_entry/sensor_actuator/
 fail-> Date type:  https://covesa.github.io/vehicle_signal_specification/rule_set/data_entry/sensor_actuator/data_unit_types/
 fail-> Struct: https://covesa.github.io/vehicle_signal_specification/rule_set/data_entry/data_types/data_types_struct


Missing fpr my app:
 unit definition pas enum
 short description (or friendly name in locale language)
 struct pointer vers un vpath pour un capteur virtuel
 struct pas vraiment clair
  - le datatype peut etre une structure (ajouter un cas struct qui pointer sur une chaine)
 instance toujours des chaines de caractères, mais des fois entouré de "

 array + arraysize sinon datasize doit imperativement être devant
 type => unit: m/s^2

 Allowed: string with '' in place "" vhehicule.vspec:166


A Faire:
 * traiter les structs
