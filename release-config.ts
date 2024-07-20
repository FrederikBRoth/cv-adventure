export default {
  afterRelease: async ({ exec, nextVersion }) => {
    await exec(`echo "${nextVersion}" > release.txt`);
  },
  beforePrepare: async ({ exec, nextVersion }) => {

    await exec(`echo "testinglmap" > file.txt`);

  },

};

