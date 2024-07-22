from pyais.stream import IterMessages


text = """
\s:2573345,c:1720601980*0F\!BSVDM,1,1,,A,13n7Qb00000kJ0vU7@CPq@e>00RD,0*4D
\s:2573135,c:1720601980*0A\!BSVDM,1,1,,B,13mN=87OiEPTIGHQ:9KHdVWB0L16,0*03
\s:2573105,c:1720601981*08\!B2VDM,1,1,9,B,33oG?>5P000i6N0RArqN4?w@0Dg:,0*72
\s:2573505,c:1720601981*0C\!BSVDM,1,1,,A,H3m6eW4QC=D76Vc<8jhkh00H:330,0*01
"""

messages = [line.encode() for line in text.split() if line]

with IterMessages(messages) as s:
    for msg in s:
        if msg.tag_block is not None:
            # Not every message has a tag block
            # Therefore, check if the tag block is not None
            # Also, it is required to call `.init()`, because tag blocks are lazily parsed
            msg.tag_block.init()
            # Print the tag block data as a dictionary
            print(msg.tag_block.asdict())
        print(msg.decode())


# message2: in rust parsed : radio_status: Sotdma(SotdmaMessage { sync_state: UtcDirect, slot_timeout: 7, sub_message: ReceivedStations(70) }) }
# in python parsed radio=114758 


#    PARSED WITH PYAIS
# {'raw': b's:2573345,c:1720601980*0F', 'receiver_timestamp': '1720601980', 'source_station': '2573345', 'destination_station': None, 'line_count': None, 'relative_time': None, 'text': None}
# MessageType1(msg_type=1, repeat=0, mmsi=258073000, status=<NavigationStatus.UnderWayUsingEngine: 0>, turn=0.0, speed=0.0, accuracy=False, lon=11.229918, lat=64.86029, course=22.9, heading=22, second=39, maneuver=<ManeuverIndicator.NotAvailable: 0>, spare_1=b'\x00', raim=False, radio=2196)
# {'raw': b's:2573135,c:1720601980*0A', 'receiver_timestamp': '1720601980', 'source_station': '2573135', 'destination_station': None, 'line_count': None, 'relative_time': None, 'text': None}
# MessageType1(msg_type=1, repeat=0, mmsi=257396000, status=<NavigationStatus.EngagedInFishing: 7>, turn=<TurnRate.NO_TI_RIGHT: 127.0>, speed=8.5, accuracy=True, lon=7.9509, lat=57.948768, course=222.6, heading=211, second=41, maneuver=<ManeuverIndicator.NotAvailable: 0>, spare_1=b'\x00', raim=False, radio=114758)
# {'raw': b's:2573105,c:1720601981*08', 'receiver_timestamp': '1720601981', 'source_station': '2573105', 'destination_station': None, 'line_count': None, 'relative_time': None, 'text': None}
# MessageType3(msg_type=3, repeat=0, mmsi=259379000, status=<NavigationStatus.Moored: 5>, turn=<TurnRate.NO_TI_DEFAULT: -128.0>, speed=0.0, accuracy=False, lon=10.726293, lat=59.908648, course=360.0, heading=511, second=40, maneuver=<ManeuverIndicator.NotAvailable: 0>, spare_1=b'\x00', raim=False, radio=84938)
#  {'raw': b's:2573505,c:1720601981*0C', 'receiver_timestamp': '1720601981', 'source_station': '2573505', 'destination_station': None, 'line_count': None, 'relative_time': None, 'text': None}
#  MessageType24PartB(msg_type=24, repeat=0, mmsi=257011100, partno=1, ship_type=33, vendorid='SMT', model=1, serial=813483, callsign='LH2030', to_bow=3, to_stern=10, to_port=3, to_starboard=3, spare_1=b'\x00')


#    PARSED WITH RUST
# Raw AIS message: \s:2573345,c:1720601980*0F\!BSVDM,1,1,,A,13n7Qb00000kJ0vU7@CPq@e>00RD,0*4D
# Parsed AIS Position Report: PositionReport { message_type: 1, repeat_indicator: 0, mmsi: 258073000, navigation_status: Some(UnderWayUsingEngine), rate_of_turn: Some(RateOfTurn { raw: 0 }), speed_over_ground: Some(0.0), position_accuracy: Unaugmented, longitude: Some(11.2299185), latitude: Some(64.86029), course_over_ground: Some(22.9), true_heading: Some(22), timestamp: 39, maneuver_indicator: None, raim: false, radio_status: Sotdma(SotdmaMessage { sync_state: UtcDirect, slot_timeout: 0, sub_message: SlotOffset(2196) }) }
# Raw AIS message: \s:2573135,c:1720601980*0A\!BSVDM,1,1,,B,13mN=87OiEPTIGHQ:9KHdVWB0L16,0*03
# Parsed AIS Position Report: PositionReport { message_type: 1, repeat_indicator: 0, mmsi: 257396000, navigation_status: Some(EngagedInFishing), rate_of_turn: Some(RateOfTurn { raw: 127 }), speed_over_ground: Some(8.5), position_accuracy: Dgps, longitude: Some(7.9509), latitude: Some(57.948765), course_over_ground: Some(222.6), true_heading: Some(211), timestamp: 41, maneuver_indicator: None, raim: false, radio_status: Sotdma(SotdmaMessage { sync_state: UtcDirect, slot_timeout: 7, sub_message: ReceivedStations(70) }) }
# Raw AIS message: \s:2573105,c:1720601981*08\!B2VDM,1,1,9,B,33oG?>5P000i6N0RArqN4?w@0Dg:,0*72
# Parsed AIS Position Report: PositionReport { message_type: 3, repeat_indicator: 0, mmsi: 259379000, navigation_status: Some(Moored), rate_of_turn: None, speed_over_ground: Some(0.0), position_accuracy: Unaugmented, longitude: Some(10.726294), latitude: Some(59.908646), course_over_ground: None, true_heading: None, timestamp: 40, maneuver_indicator: None, raim: false, radio_status: Itdma(ItdmaMessage { sync_state: UtcDirect, slot_increment: 5308, num_slots: 5, keep: false }) }
# Raw AIS message: \s:2573505,c:1720601981*0C\!BSVDM,1,1,,A,H3m6eW4QC=D76Vc<8jhkh00H:330,0*01
# Parsed AIS Static Data Report: StaticDataReport { message_type: 24, repeat_indicator: 0, mmsi: 257011100, message_part: PartB { ship_type: Some(Dredging), vendor_id: "SMT", model_serial: "GF&+", unit_model_code: 1, serial_number: 813483, callsign: "LH2030", dimension_to_bow: 3, dimension_to_stern: 10, dimension_to_port: 3, dimension_to_starboard: 3 } }
