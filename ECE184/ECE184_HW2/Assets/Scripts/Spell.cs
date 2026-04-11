using System;
using UnityEngine;

public class Spell : MonoBehaviour {
    [SerializeField] private float dps = 5f;
    public bool casting = false;
    
    private ParticleSystem _particleSystem;

    private void Awake() {
        _particleSystem = GetComponent<ParticleSystem>();
    }

    private void Update() {
        var emission = _particleSystem.emission;
        emission.enabled = casting;
    }

    private void OnTriggerStay(Collider other) {
        if (!casting) return;
        if (other.gameObject.TryGetComponent<Health>(out var health)) {
            health.Damage(Time.deltaTime * dps);
        }
    }
}