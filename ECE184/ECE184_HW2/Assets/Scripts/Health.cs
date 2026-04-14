using UnityEngine;

public class Health : MonoBehaviour {
    public float initialHealth = 10f;
    public float health;

    private void Awake() {
        health = initialHealth;
    }
    
    public void Damage(float amount) {
        health -= amount;
    }
    
    public void Heal(float amount) {
        health += amount;
    }
}