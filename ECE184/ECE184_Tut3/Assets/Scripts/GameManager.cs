using System;
using UnityEngine;

public class GameManager : MonoBehaviour {
    public static GameManager instance;

    private int coins = 0;
    public event EventHandler<int> coinsChanged;
    
    public void Awake() {
        if (instance == null) {
            instance = this;
        }
        else {
            Destroy(gameObject);
        }
    }

    public void CollectCoin(int n) {
        coins += n;
        coinsChanged?.Invoke(this, coins);
    }
}