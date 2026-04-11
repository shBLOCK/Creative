using UnityEngine;

public class Health : MonoBehaviour {
    public float health { get; private set; } = 10f;
    
    public void Damage(float amount) {
        health -= amount;
    }
    
    public void Heal(float amount) {
        health += amount;
    }
}