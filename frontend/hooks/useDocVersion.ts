import { useState, useEffect } from 'react';
import { DocVersion } from '../types/docs';
import versionsData from '../data/versions.json';

const realVersions: DocVersion[] = versionsData as DocVersion[];

export function useDocVersion() {
  const [selectedVersion, setSelectedVersion] = useState<DocVersion>(realVersions[0]);

  // Load from local storage on mount
  useEffect(() => {
    const stored = localStorage.getItem('soroban-zk-doc-version');
    if (stored) {
      const found = realVersions.find((v) => v.id === stored);
      if (found) {
        setSelectedVersion(found);
      }
    }
  }, []);

  const setVersion = (version: DocVersion) => {
    setSelectedVersion(version);
    localStorage.setItem('soroban-zk-doc-version', version.id);
  };

  return {
    versions: realVersions,
    selectedVersion,
    setVersion,
  };
}
