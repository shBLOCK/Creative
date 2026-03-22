extends Node


@onready var socket := PacketPeerUDP.new()

func _ready() -> void:
	socket.bind(30002)

func _dict_to_vec3(d: Dictionary) -> Vector3:
	return Vector3(d["x"], d["y"], d["z"])

func _process(_delta: float) -> void:
	while socket.get_available_packet_count():
		var pkt := JSON.parse_string(socket.get_packet().get_string_from_ascii()) as Dictionary
		var left_eye_3d := _dict_to_vec3(pkt["left_eye_3d"])
		var right_eye_3d := _dict_to_vec3(pkt["right_eye_3d"])
		$RealWorld/Camera/Eye.position = (left_eye_3d + right_eye_3d) / 2 * Vector3(1, 1, -1)	
	
	var screen := DisplayServer.get_primary_screen()
	var screen_size_pixel := DisplayServer.screen_get_size(screen)
	var screen_center_pixel := Vector2(DisplayServer.screen_get_position(screen)) + screen_size_pixel / 2.0
	var screen_pixel_scale := 21.5 / screen_size_pixel.y # pixel to centimeters
	var window := get_window()
	var window_size_pixel := DisplayServer.window_get_size(window.get_window_id())
	var window_center_pixel := Vector2(DisplayServer.window_get_position(window.get_window_id())) + window_size_pixel / 2.0 - screen_center_pixel
	var window_size := window_size_pixel * screen_pixel_scale
	var window_center := window_center_pixel * screen_pixel_scale * Vector2(1.0, -1.0)
	%Viewport.position = Vector3(window_center.x, window_center.y, 0.0)
	%Viewport.v_size = window_size.y
	%Viewport.real_transform.origin = Vector3(window_center.x, window_center.y, 0.0)
	%Viewport.real_v_size = window_size.y
	%Root.position = Vector3(window_center.x, window_center.y, 0.0)
	

func _input(event: InputEvent) -> void:
	if event is InputEventMouseButton:
		if event.button_index == MOUSE_BUTTON_LEFT and event.is_pressed():
			get_window().start_drag()
			pass
