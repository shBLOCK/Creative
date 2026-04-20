using System;
using UnityEngine;
using UnityEngine.Events;

public class Trigger : MonoBehaviour {
    public UnityEvent<bool> stateChanged;

    private void OnTriggerEnter(Collider other) {
        stateChanged.Invoke(true);
    }
    
    private void OnTriggerExit(Collider other) {
        stateChanged.Invoke(false);
    }
}