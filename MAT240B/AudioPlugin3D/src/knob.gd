extends Area3D

var value := 0.0
@export var target_value := 0.0

@export var min_value := 0.0
@export var max_value := 1.0

func _ready() -> void:
	value = target_value

func _physics_process(delta: float) -> void:
	value = lerp(value, target_value, 0.1)
	$Model.rotation.z = -remap(value, min_value, max_value, 0.1, 0.9) * TAU

func _process(delta: float) -> void:
	$Label3D.text = "%.2f" % value

func _on_input_event(camera: Node, event: InputEvent, event_position: Vector3, normal: Vector3, shape_idx: int) -> void:
	if event is InputEventMouseButton:
		if event.is_pressed():
			const SPEED := 0.05
			if event.button_index == MOUSE_BUTTON_WHEEL_UP:
				target_value = min(target_value + SPEED * (max_value - min_value), max_value)
			if event.button_index == MOUSE_BUTTON_WHEEL_DOWN:
				target_value = max(target_value - SPEED * (max_value - min_value), min_value)
