using UnityEngine;

public class MainMenu : MonoBehaviour {
    [SerializeField] private GameObject[] objectsToActivate;
    
    public void startPressed() {
        gameObject.SetActive(false);
        foreach (var obj in objectsToActivate) {
            obj.SetActive(true);
        }
    }
}
