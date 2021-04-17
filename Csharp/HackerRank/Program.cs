using System;
using System.Collections.Generic;

namespace HackerRank
{
    class Program
    {
        static void Main(string[] args)
        {
            NotesStore notes = new NotesStore();
            notes.AddNote("completed", "Hello");
            notes.AddNote("completed", "Hello2");
            notes.AddNote("completed", "Hello3");
            notes.AddNote("active", "Hello4");
            notes.AddNote("completed", "Hello5");
            notes.AddNote("others", "Hello5");
            notes.AddNote("othe3rs", "Hello5");
            var newNotes = notes.GetNotes("completed");
            foreach (var note in newNotes)
            {
                Console.WriteLine(note);
            }
        }

        public class NotesStore
        {
            public Dictionary<String, List<String>> stateDict = new Dictionary<String, List<String>>();

            public NotesStore() { }

            public void AddNote(String state, String name)
            {
                if (String.IsNullOrEmpty(name))
                {
                    throw new Exception("Name cannot be empty");
                }
                if (state != "completed" && state != "active" && state != "others")
                {
                    throw new Exception($"Invalid state {state}");
                }

                if (this.stateDict.ContainsKey(state))
                {
                    var values = this.stateDict[state];
                    values.Add(name);
                    this.stateDict.Remove(state);
                    this.stateDict.Add(state, values);
                    return;
                }

                List<String> newVal = new List<String> { name };

                this.stateDict.Add(state, newVal);
            }

            public List<String> GetNotes(String state)
            {
                if (state != "completed" && state != "active" && state != "others")
                {
                    throw new Exception($"Invalid state {state}");
                }

                if (!this.stateDict.ContainsKey("completed") && !this.stateDict.ContainsKey("active") && !this.stateDict.ContainsKey("others"))
                {
                    return new List<string>();
                }

                return this.stateDict[state];
            }
        }

        public class Team
        {
            public string teamName;
            public int noOfPlayers;

            public Team(string teamName, int noOfPlayer)
            {
                this.teamName = teamName;
                this.noOfPlayers = noOfPlayer;
            }

            public void AddPlayer(int count)
            {
                this.noOfPlayers += count;
            }

            public bool RemovePlayer(int count)
            {
                var _noOfPlayer = this.noOfPlayers - count;
                if (_noOfPlayer < 0)
                {
                    return false;
                }

                this.noOfPlayers = _noOfPlayer;
                return true;
            }
        }

        public class Subteam : Team
        {
            public Subteam(string teamName, int noOfPlayers) : base(teamName, noOfPlayers)
            {

            }

            public void ChangeTeamName(string name)
            {
                this.teamName = name;
            }

        }
    }
}
