using System;
using TMPro;
using Unity.Mathematics;
using UnityEngine;
using UnityEngine.InputSystem;

public class PlayerController : MonoBehaviour {
    [SerializeField] private float moveVelocity = 8f;
    [SerializeField] private float moveAcceleration = 5f;
    [SerializeField] private float jumpSpeed = 10f;

    private InputAction _moveAction;
    private InputAction _sprintAction;
    private InputAction _jumpAction;
    private InputAction _lookAction;
    private InputAction _castAction;
    private Rigidbody _rigidBody;
    private Animator _animator;
    private Spell _spell;

    void Start() {
        _moveAction = InputSystem.actions.FindAction("Player/Move");
        _sprintAction = InputSystem.actions.FindAction("Player/Sprint");
        _jumpAction = InputSystem.actions.FindAction("Player/Jump");
        _jumpAction.performed += _ => {
            _rigidBody.AddForce(Vector3.up * jumpSpeed, ForceMode.VelocityChange);
            _animator.SetTrigger("Jump");
        };
        _lookAction = InputSystem.actions.FindAction("Player/Look");
        _castAction = InputSystem.actions.FindAction("Player/Attack");
        _rigidBody = GetComponent<Rigidbody>();
        _animator = GetComponent<Animator>();
        _spell = GetComponentInChildren<Spell>();
    }

    private void Update() {
        _animator.SetFloat(
            "Speed",
            _rigidBody.linearVelocity.magnitude / moveVelocity *
            math.sign(Vector3.Dot(_rigidBody.linearVelocity, transform.forward))
        );

        var casting = _castAction.IsPressed();
        _spell.casting = casting;
        _animator.SetBool("Cast", casting);
    }

    private void FixedUpdate() {
        var targetVel = _moveAction.ReadValue<Vector2>() * (moveVelocity * (_sprintAction.IsPressed() ? 1f : 0.5f));
        var currentVel = new Vector2(_rigidBody.linearVelocity.x, _rigidBody.linearVelocity.z);
        var force = (targetVel - currentVel) * moveAcceleration;
        _rigidBody.AddForce(new Vector3(force.x, 0, force.y), ForceMode.Acceleration);
        
        var ray = Camera.main.ScreenPointToRay(_lookAction.ReadValue<Vector2>());
        if (Physics.Raycast(ray, out var hitInfo)) {
            transform.LookAt(new Vector3(hitInfo.point.x, transform.position.y, hitInfo.point.z));
        }
    }
}