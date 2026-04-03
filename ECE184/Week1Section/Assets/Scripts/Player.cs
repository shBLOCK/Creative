using System;
using UnityEngine;
using UnityEngine.InputSystem;

public class Player : MonoBehaviour {
    [SerializeField] private float moveVelocity = 8f;
    [SerializeField] private float moveForce = 5f;
    [SerializeField] private float jumpForce = 8f;
    [SerializeField] private float upRightTorque = 100f;

    private MeshRenderer meshRenderer;
    private Rigidbody rigidBody;
    private InputAction moveAction;
    private InputAction jumpAction;
    private bool onGround = false;
    
    void Start() {
        meshRenderer = GetComponent<MeshRenderer>();
        rigidBody = GetComponent<Rigidbody>();
        moveAction = InputSystem.actions.FindAction("Player/Move");
        jumpAction = InputSystem.actions.FindAction("Player/Jump");
        jumpAction.performed += _ => {
            if (onGround) {
                rigidBody.AddForce(Vector3.up * jumpForce, ForceMode.VelocityChange);
            }
        };
    }

    void Update() {
        meshRenderer.material.color = onGround ? Color.green : Color.red;
    }

    private void FixedUpdate() {
        onGround = Physics.OverlapSphere(transform.position + Vector3.down * 1f, 0.2f).Length > 1;
        
        var targetVel = moveAction.ReadValue<Vector2>() * moveVelocity;
        var currentVel = new Vector2(rigidBody.linearVelocity.x, rigidBody.linearVelocity.z);
        var force = (targetVel - currentVel) * moveForce;
        rigidBody.AddForce(new Vector3(force.x, 0, force.y), ForceMode.Acceleration);
        
        rigidBody.AddTorque(Vector3.Cross(transform.up, Vector3.up) * upRightTorque, ForceMode.Acceleration);
    }
}