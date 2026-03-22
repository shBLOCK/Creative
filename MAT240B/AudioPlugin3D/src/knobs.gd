extends Node3D

func _modulated(value, modulation) -> Dictionary:
	return {"value": value, "modulation": modulation}

func _make_packet() -> Dictionary:
	return {
		"volume": _modulated($Volume.value, 0.0),
		"adsr": {
			"attack_duration": _modulated($Attack.value, 0.0),
			"attack_power": _modulated($AttackCurve.value, 0.0),
			"decay_duration": _modulated($Decay.value, 0.0),
			"decay_power": _modulated($DecayCurve.value, 0.0),
			"sustain": _modulated($Sustain.value, 0.0),
			"release_duration": _modulated($Release.value, 0.0),
			"release_power": _modulated($ReleaseCurve.value, 0.0),
		},
		"hf_rolloff": _modulated($"HF Rolloff".value, 0.0),
	}

@onready var socket := PacketPeerUDP.new()

func _ready() -> void:
	socket.connect_to_host("127.0.0.1", 30100)

func _physics_process(delta: float) -> void:
	var pkt := _make_packet()
	#print(pkt)
	socket.put_packet(JSON.stringify(pkt).to_ascii_buffer())
	