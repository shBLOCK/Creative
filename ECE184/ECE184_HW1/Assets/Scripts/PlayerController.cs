using System;
using TMPro;
using UnityEngine;
using UnityEngine.InputSystem;

public class PlayerController : MonoBehaviour {
    [SerializeField] private float moveVelocity = 8f;
    [SerializeField] private float moveAcceleration = 5f;
    [SerializeField] private GameObject bulletPrefab;

    private InputAction moveAction;
    private InputAction lookAction;
    private InputAction fireAction;
    private Rigidbody rigidBody;

    public int score = 0;
    [SerializeField] private TMP_Text scoreText;

    void Start() {
        moveAction = InputSystem.actions.FindAction("Player/Move");
        lookAction = InputSystem.actions.FindAction("Player/Look");
        fireAction = InputSystem.actions.FindAction("Player/Attack");
        rigidBody = GetComponent<Rigidbody>();
    }

    private void Update() {
        if (fireAction.triggered) {
            Instantiate(bulletPrefab, transform.position, transform.rotation);
        }
    }

    private void FixedUpdate() {
        var targetVel = moveAction.ReadValue<Vector2>() * moveVelocity;
        var currentVel = new Vector2(rigidBody.linearVelocity.x, rigidBody.linearVelocity.z);
        var force = (targetVel - currentVel) * moveAcceleration;
        rigidBody.AddForce(new Vector3(force.x, 0, force.y), ForceMode.Acceleration);

        var ray = Camera.main.ScreenPointToRay(lookAction.ReadValue<Vector2>());
        if (Physics.Raycast(ray, out var hitInfo)) {
            transform.LookAt(new Vector3(hitInfo.point.x, transform.position.y, hitInfo.point.z));
        }
    }
    
    public void AddScore(int points) {
        score += points;
        scoreText.text = $"Score: {score}";
    }
}